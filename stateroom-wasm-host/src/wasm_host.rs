use crate::WasmRuntimeError;
use anyhow::{Context, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use stateroom::{
    ClientId, MessageFromProcess, MessagePayload, MessageToProcess, StateroomContext,
    StateroomService,
};
use std::{borrow::BorrowMut, sync::Arc};
use wasi_common::{sync::WasiCtxBuilder, WasiCtx};
use wasmtime::{Caller, Engine, Extern, Instance, Linker, Memory, Module, Store, TypedFunc, Val};

const ENV: &str = "env";
const EXT_MEMORY: &str = "memory";
const EXT_FN_SEND: &str = "stateroom_send";
const EXT_FN_RECV: &str = "stateroom_recv";
const EXT_FN_MALLOC: &str = "stateroom_malloc";
const EXT_FN_FREE: &str = "stateroom_free";
const EXT_STATEROOM_VERSION: &str = "STATEROOM_API_VERSION";
const EXT_STATEROOM_PROTOCOL: &str = "STATEROOM_API_PROTOCOL";

const EXPECTED_API_VERSION: i32 = 1;
const EXPECTED_PROTOCOL_VERSION: i32 = 0;

/// Hosts a [stateroom::StateroomService] implemented by a WebAssembly module.
pub struct WasmHost {
    store: Store<WasiCtx>,
    memory: Memory,

    fn_malloc: TypedFunc<u32, u32>,
    fn_free: TypedFunc<(u32, u32), ()>,
    fn_recv: TypedFunc<(u32, u32), ()>,
}

impl WasmHost {
    fn put_data(&mut self, data: &[u8]) -> Result<(u32, u32)> {
        #[allow(clippy::cast_possible_truncation)]
        let len = data.len() as u32;
        let pt = self.fn_malloc.call(&mut self.store, len)?;

        self.memory.write(&mut self.store, pt as usize, data)?;

        Ok((pt, len))
    }

    fn try_recv(&mut self, message: MessageToProcess) -> Result<()> {
        let payload = bincode::serialize(&message).unwrap();
        let (pt, len) = self.put_data(&payload)?;

        self.fn_recv.call(&mut self.store, (pt, len))?;
        self.fn_free.call(&mut self.store, (pt, len))?;

        Ok(())
    }
}

impl StateroomService for WasmHost {
    fn init(&mut self, _: &impl StateroomContext) {
        let message = MessageToProcess::Init;
        self.try_recv(message).unwrap();
    }

    fn message(&mut self, sender: ClientId, message: MessagePayload, _: &impl StateroomContext) {
        let message = MessageToProcess::Message { sender, message };
        self.try_recv(message).unwrap();
    }

    fn connect(&mut self, client: ClientId, _: &impl StateroomContext) {
        let message = MessageToProcess::Connect { client };
        self.try_recv(message).unwrap();
    }

    fn disconnect(&mut self, client: ClientId, _: &impl StateroomContext) {
        let message = MessageToProcess::Disconnect { client };
        self.try_recv(message).unwrap();
    }

    fn timer(&mut self, _: &impl StateroomContext) {
        let message = MessageToProcess::Timer;
        self.try_recv(message).unwrap();
    }
}

#[inline]
fn get_memory<T>(caller: &mut Caller<'_, T>) -> Memory {
    match caller.get_export(EXT_MEMORY) {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!(),
    }
}

#[inline]
fn get_u8_vec<'a, T>(
    caller: &'a Caller<'_, T>,
    memory: &'a Memory,
    start: u32,
    len: u32,
) -> &'a [u8] {
    let data = memory
        .data(caller)
        .get(start as usize..(start + len) as usize);
    match data {
        Some(data) => data,
        None => panic!(),
    }
}

pub fn get_global<T>(
    store: &mut Store<T>,
    memory: &mut Memory,
    instance: &Instance,
    name: &str,
) -> Result<i32> {
    #[allow(clippy::cast_sign_loss)]
    let i: u32 = {
        let mem_location = instance
            .get_global(store.borrow_mut(), name)
            .ok_or(WasmRuntimeError::CouldNotImportGlobal)?;

        match mem_location.get(store.borrow_mut()) {
            Val::I32(i) => Ok(i),
            _ => Err(WasmRuntimeError::CouldNotImportGlobal),
        }? as u32
    };

    #[allow(clippy::cast_possible_truncation)]
    let mut value = memory
        .data(store)
        .get(i as usize..(i as usize + std::mem::size_of::<i32>()))
        .ok_or(WasmRuntimeError::CouldNotImportGlobal)?;
    let result = value.read_i32::<LittleEndian>()?;
    Ok(result)
}

impl WasmHost {
    pub fn new(
        room_id: &str,
        module: &Module,
        engine: &Engine,
        context: Arc<impl StateroomContext>,
    ) -> Result<Self> {
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();

        let mut store = Store::new(engine, wasi);
        let mut linker = Linker::new(engine);
        wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

        {
            #[allow(clippy::redundant_clone)]
            let context = context.clone();
            linker.func_wrap(
                ENV,
                EXT_FN_SEND,
                move |mut caller: Caller<'_, WasiCtx>, start: u32, len: u32| {
                    let memory = get_memory(&mut caller);
                    let message = get_u8_vec(&caller, &memory, start, len);
                    let message: MessageFromProcess = bincode::deserialize(message).unwrap();

                    match message {
                        MessageFromProcess::Message { recipient, message } => {
                            context.send_message(recipient, message);
                        }
                        MessageFromProcess::SetTimer { ms_delay } => {
                            context.set_timer(ms_delay);
                        }
                    };

                    Ok(())
                },
            )?;
        }

        let instance = linker.instantiate(&mut store, module)?;

        let fn_malloc = instance.get_typed_func::<u32, u32>(&mut store, EXT_FN_MALLOC)?;

        let fn_free = instance.get_typed_func::<(u32, u32), ()>(&mut store, EXT_FN_FREE)?;

        let fn_recv = instance.get_typed_func::<(u32, u32), ()>(&mut store, EXT_FN_RECV)?;

        let mut memory = instance
            .get_memory(&mut store, EXT_MEMORY)
            .ok_or(WasmRuntimeError::CouldNotImportMemory)?;

        {
            let room_id = room_id.as_bytes();
            #[allow(clippy::cast_possible_truncation)]
            let len = room_id.len() as u32;
            let pt = fn_malloc.call(&mut store, len)?;

            memory.write(&mut store, pt as usize, room_id)?;

            fn_free.call(&mut store, (pt, len))?;
        }

        if get_global(&mut store, &mut memory, &instance, EXT_STATEROOM_VERSION)
            .context("Stateroom version")?
            != EXPECTED_API_VERSION
        {
            return Err(WasmRuntimeError::InvalidApiVersion.into());
        }

        if get_global(&mut store, &mut memory, &instance, EXT_STATEROOM_PROTOCOL)
            .context("Stateroom protocol")?
            != EXPECTED_PROTOCOL_VERSION
        {
            return Err(WasmRuntimeError::InvalidProtocolVersion.into());
        }

        Ok(WasmHost {
            store,
            memory,
            fn_malloc,
            fn_free,
            fn_recv,
        })
    }
}
