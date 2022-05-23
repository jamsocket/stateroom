use crate::WasmRuntimeError;
use anyhow::Result;
use async_trait::async_trait;
use byteorder::{LittleEndian, ReadBytesExt};
use stateroom::MessagePayload;
use stateroom::{MessageFromRoom, MessageRecipient, RoomEvent, Stateroom, StateroomContext};
use std::borrow::BorrowMut;
use std::path::Path;
use tokio::sync::mpsc::{channel, Receiver};
use wasmtime::{Caller, Engine, Extern, Instance, Linker, Memory, Module, Store, TypedFunc, Val};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

const ENV: &str = "env";
const EXT_MEMORY: &str = "memory";
const EXT_FN_MESSAGE: &str = "message";
const EXT_FN_SEND_MESSAGE: &str = "send_message";
const EXT_FN_SEND_BINARY: &str = "send_binary";
const EXT_FN_INITIALIZE: &str = "initialize";
const EXT_FN_POLL: &str = "poll";
const EXT_FN_MALLOC: &str = "stateroom_malloc";
const EXT_FN_FREE: &str = "stateroom_free";
const EXT_STATEROOM_VERSION: &str = "STATEROOM_API_VERSION";
const EXT_STATEROOM_PROTOCOL: &str = "STATEROOM_API_PROTOCOL";

const EXPECTED_API_VERSION: i32 = 1;
const EXPECTED_PROTOCOL_VERSION: i32 = 1;

/// Hosts a [stateroom::StateroomService] implemented by a WebAssembly module.
pub struct WasmHost {
    store: Store<WasiCtx>,
    memory: Memory,
    message_receiver: Option<Receiver<MessageFromRoom>>,

    fn_malloc: TypedFunc<u32, u32>,
    fn_free: TypedFunc<(u32, u32), ()>,
    fn_message: TypedFunc<(u32, u32), ()>,
    fn_poll: TypedFunc<(), ()>,
}

impl WasmHost {
    fn put_data(&mut self, data: &[u8]) -> Result<(u32, u32)> {
        #[allow(clippy::cast_possible_truncation)]
        let len = data.len() as u32;
        let pt = self.fn_malloc.call(&mut self.store, len)?;

        self.memory.write(&mut self.store, pt as usize, data)?;

        Ok((pt, len))
    }

    fn try_message(&mut self, message: RoomEvent) -> Result<()> {
        let (pt, len) = self
            .put_data(&bincode::serialize(&message).expect("Error serializing message."))
            .expect("Error putting data.");

        self.fn_message
            .call(&mut self.store, (pt, len))
            .expect("Error calling message");

        self.fn_free
            .call(&mut self.store, (pt, len))
            .expect("Error calling free");

        Ok(())
    }
}

#[async_trait]
impl Stateroom for WasmHost {
    async fn run<C: StateroomContext>(mut self, mut ctx: C) -> ()
    where
        C: 'async_trait + StateroomContext,
    {
        let mut receiver = self.message_receiver.take().unwrap();
        self.fn_poll
            .call(&mut self.store, ())
            .expect("Initial poll failed.");

        loop {
            tokio::select! {
                message = ctx.next_event() => {
                    self.try_message(message).expect("Error on message.");
                    self.fn_poll.call(&mut self.store, ()).expect("Poll after message failed.");
                },
                message = receiver.recv() => {
                    let MessageFromRoom::Message {message, recipient} = message.expect("Receiver closed unexpectedly.");
                    match message {
                        MessagePayload::Bytes(b) => ctx.send_binary(recipient, &b),
                        MessagePayload::Text(t) => ctx.send(recipient, &t),
                    }
                }
            }
        }
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
fn get_string<'a, T>(
    caller: &'a Caller<'_, T>,
    memory: &'a Memory,
    start: u32,
    len: u32,
) -> Result<&'a str> {
    let data = get_u8_vec(caller, memory, start, len);
    std::str::from_utf8(data).map_err(|e| e.into())
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
    pub fn new<P>(wasm_file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let engine = Engine::default();
        tracing::info!(wasm_file=?wasm_file.as_ref(), "Loading WebAssembly module");
        let module = Module::from_file(&engine, wasm_file)?;

        Self::new_with_module_engine(&module, &engine)
    }

    pub fn new_with_module_engine(module: &Module, engine: &Engine) -> Result<Self> {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();

        let mut store = Store::new(engine, wasi);
        let mut linker = Linker::new(engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let (sender, receiver) = channel(256);

        {
            #[allow(clippy::redundant_clone)]
            let sender = sender.clone();
            linker.func_wrap(
                ENV,
                EXT_FN_SEND_MESSAGE,
                move |mut caller: Caller<'_, WasiCtx>, client: u32, start: u32, len: u32| {
                    let memory = get_memory(&mut caller);
                    let message = get_string(&caller, &memory, start, len)?;

                    sender
                        .try_send(MessageFromRoom::Message {
                            recipient: MessageRecipient::decode_u32(client),
                            message: MessagePayload::Text(message.to_string()),
                        })
                        .unwrap();

                    Ok(())
                },
            )?;
        }

        {
            #[allow(clippy::redundant_clone)]
            let sender = sender.clone();
            linker.func_wrap(
                ENV,
                EXT_FN_SEND_BINARY,
                move |mut caller: Caller<'_, WasiCtx>, client: u32, start: u32, len: u32| {
                    let memory = get_memory(&mut caller);
                    let message = get_u8_vec(&caller, &memory, start, len);

                    sender
                        .try_send(MessageFromRoom::Message {
                            recipient: MessageRecipient::decode_u32(client),
                            message: MessagePayload::Bytes(message.to_vec()),
                        })
                        .unwrap();

                    Ok(())
                },
            )?;
        }

        let instance = linker.instantiate(&mut store, module)?;

        let initialize = instance.get_typed_func::<(), (), _>(&mut store, EXT_FN_INITIALIZE)?;

        let fn_poll = instance.get_typed_func::<(), (), _>(&mut store, EXT_FN_POLL)?;

        let fn_malloc = instance.get_typed_func::<u32, u32, _>(&mut store, EXT_FN_MALLOC)?;

        let fn_free = instance.get_typed_func::<(u32, u32), (), _>(&mut store, EXT_FN_FREE)?;

        let mut memory = instance
            .get_memory(&mut store, EXT_MEMORY)
            .ok_or(WasmRuntimeError::CouldNotImportMemory)?;

        if get_global(&mut store, &mut memory, &instance, EXT_STATEROOM_VERSION)?
            != EXPECTED_API_VERSION
        {
            return Err(WasmRuntimeError::InvalidApiVersion.into());
        }

        if get_global(&mut store, &mut memory, &instance, EXT_STATEROOM_PROTOCOL)?
            != EXPECTED_PROTOCOL_VERSION
        {
            return Err(WasmRuntimeError::InvalidProtocolVersion.into());
        }

        initialize.call(&mut store, ())?;

        let fn_message =
            instance.get_typed_func::<(u32, u32), (), _>(&mut store, EXT_FN_MESSAGE)?;

        Ok(WasmHost {
            store,
            memory,
            fn_malloc,
            fn_free,
            fn_message,
            fn_poll,
            message_receiver: Some(receiver),
        })
    }
}
