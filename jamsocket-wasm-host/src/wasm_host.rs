use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use jamsocket::{JamsocketContext, JamsocketService, MessageRecipient};
use std::{borrow::BorrowMut, sync::Arc};
use wasmtime::{Caller, Engine, Extern, Instance, Linker, Memory, Module, Store, TypedFunc, Val};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

use crate::WasmRuntimeError;

const ENV: &str = "env";
const EXT_MEMORY: &str = "memory";
const EXT_FN_CONNECT: &str = "connect";
const EXT_FN_DISCONNECT: &str = "disconnect";
const EXT_FN_BINARY: &str = "binary";
const EXT_FN_MESSAGE: &str = "message";
const EXT_FN_SEND_MESSAGE: &str = "send_message";
const EXT_FN_SET_TIMER: &str = "set_timer";
const EXT_FN_TIMER: &str = "timer";
const EXT_FN_INITIALIZE: &str = "initialize";
const EXT_FN_MALLOC: &str = "malloc";
const EXT_FN_FREE: &str = "free";
const EXT_JAMSOCKET_VERSION: &str = "JAMSOCKET_API_VERSION";
const EXT_JAMSOCKET_PROTOCOL: &str = "JAMSOCKET_API_PROTOCOL";

const EXPECTED_API_VERSION: i32 = 1;
const EXPECTED_PROTOCOL_VERSION: i32 = 0;

/// Hosts a [jamsocket::JamsocketService] implemented by a WebAssembly module.
pub struct WasmHost {
    store: Store<WasiCtx>,
    memory: Memory,

    fn_malloc: TypedFunc<u32, u32>,
    fn_free: TypedFunc<(u32, u32), ()>,
    fn_message: TypedFunc<(u32, u32, u32), ()>,
    fn_binary: TypedFunc<(u32, u32, u32), ()>,
    fn_connect: TypedFunc<u32, ()>,
    fn_disconnect: TypedFunc<u32, ()>,
    fn_timer: TypedFunc<(), ()>,
}

impl WasmHost {
    pub fn try_message(&mut self, user: u32, message: &str) -> Result<()> {
        let pt = self.fn_malloc.call(&mut self.store, message.len() as u32)?;

        self.memory
            .write(&mut self.store, pt as usize, &message.as_bytes())?;

        self.fn_message
            .call(&mut self.store, (user, pt as u32, message.len() as u32))?;

        self.fn_free
            .call(&mut self.store, (pt, message.len() as u32))?;

        Ok(())
    }

    pub fn try_binary(&mut self, user: u32, message: &[u8]) -> Result<()> {
        let pt = self.fn_malloc.call(&mut self.store, message.len() as u32)?;

        self.memory.write(&mut self.store, pt as usize, &message)?;

        self.fn_binary
            .call(&mut self.store, (user, pt as u32, message.len() as u32))?;

        self.fn_free
            .call(&mut self.store, (pt, message.len() as u32))?;

        Ok(())
    }
}

impl JamsocketService for WasmHost {
    fn message(&mut self, user: u32, message: &str) {
        if let Err(e) = self.try_message(user, message) {
            println!("Error calling `message` on wasm host. {:?}", &e);
        }
    }

    fn connect(&mut self, user: u32) {
        if let Err(e) = self.fn_connect.call(&mut self.store, user) {
            println!("Error calling `connect` on wasm host. {:?}", &e);
        }
    }

    fn disconnect(&mut self, user: u32) {
        if let Err(e) = self.fn_disconnect.call(&mut self.store, user) {
            println!("Error calling `disconnect` on wasm host. {:?}", &e);
        };
    }

    fn timer(&mut self) {
        if let Err(e) = self.fn_timer.call(&mut self.store, ()) {
            println!("Error calling `timer` on wasm host. {:?}", &e);
        };
    }

    fn binary(&mut self, user: u32, message: &[u8]) {
        if let Err(e) = self.try_binary(user, message) {
            println!("Error calling `binary` on wasm host. {:?}", &e);
        };
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
) -> &'a str {
    let data = memory
        .data(caller)
        .get(start as usize..(start + len) as usize);
    match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => panic!(),
        },
        None => panic!(),
    }
}

pub fn get_global<T>(
    store: &mut Store<T>,
    memory: &mut Memory,
    instance: &Instance,
    name: &str,
) -> Result<i32> {
    let i = {
        let mem_location = instance
            .get_global(store.borrow_mut(), name)
            .ok_or(WasmRuntimeError::CouldNotImportGlobal)?;

        match mem_location.get(store.borrow_mut()) {
            Val::I32(i) => Ok(i),
            _ => Err(WasmRuntimeError::CouldNotImportGlobal),
        }?
    };

    let mut value = memory
        .data(store)
        .get(i as usize..(i as usize + std::mem::size_of::<i32>()))
        .ok_or(WasmRuntimeError::CouldNotImportGlobal)?;
    let result = value.read_i32::<LittleEndian>()?;
    Ok(result)
}

impl WasmHost {
    pub fn new(
        _token: &str,
        module: &Module,
        engine: &Engine,
        context: Arc<impl JamsocketContext + Send + Sync + 'static>,
    ) -> Result<Self> {
        let wasi = WasiCtxBuilder::new().build();

        let mut store = Store::new(&engine, wasi);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        {
            #[allow(clippy::redundant_clone)]
            let context = context.clone();
            linker.func_wrap(
                ENV,
                EXT_FN_SEND_MESSAGE,
                move |mut caller: Caller<'_, WasiCtx>, user: u32, start: u32, len: u32| {
                    let memory = get_memory(&mut caller);
                    let message = get_string(&caller, &memory, start, len);

                    context.send_message(MessageRecipient::decode_u32(user), message);

                    Ok(())
                },
            )?;
        }

        {
            #[allow(clippy::redundant_clone)]
            let context = context.clone();
            linker.func_wrap(
                ENV,
                EXT_FN_SET_TIMER,
                move |_: Caller<'_, WasiCtx>, duration_ms: u32| {
                    context.set_timer(duration_ms);

                    Ok(())
                },
            )?;
        }

        let instance = linker.instantiate(&mut store, &module)?;

        let initialize = instance.get_typed_func::<(), (), _>(&mut store, EXT_FN_INITIALIZE)?;

        // TODO: pass token to initialize.
        initialize.call(&mut store, ())?;

        let mut memory = instance
            .get_memory(&mut store, EXT_MEMORY)
            .ok_or(WasmRuntimeError::CouldNotImportMemory)?;

        if get_global(&mut store, &mut memory, &instance, EXT_JAMSOCKET_VERSION)?
            != EXPECTED_API_VERSION
        {
            return Err(WasmRuntimeError::InvalidApiVersion.into());
        }

        if get_global(&mut store, &mut memory, &instance, EXT_JAMSOCKET_PROTOCOL)?
            != EXPECTED_PROTOCOL_VERSION
        {
            return Err(WasmRuntimeError::InvalidProtocolVersion.into());
        }

        let fn_connect = instance.get_typed_func::<u32, (), _>(&mut store, EXT_FN_CONNECT)?;

        let fn_disconnect = instance.get_typed_func::<u32, (), _>(&mut store, EXT_FN_DISCONNECT)?;

        let fn_malloc = instance.get_typed_func::<u32, u32, _>(&mut store, EXT_FN_MALLOC)?;

        let fn_free = instance.get_typed_func::<(u32, u32), (), _>(&mut store, EXT_FN_FREE)?;

        let fn_timer = instance.get_typed_func::<(), (), _>(&mut store, EXT_FN_TIMER)?;

        let fn_message =
            instance.get_typed_func::<(u32, u32, u32), (), _>(&mut store, EXT_FN_MESSAGE)?;

        let fn_binary =
            instance.get_typed_func::<(u32, u32, u32), (), _>(&mut store, EXT_FN_BINARY)?;

        Ok(WasmHost {
            store,
            memory,
            fn_malloc,
            fn_free,
            fn_message,
            fn_connect,
            fn_disconnect,
            fn_timer,
            fn_binary,
        })
    }
}
