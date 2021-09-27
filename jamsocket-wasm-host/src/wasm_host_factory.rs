use crate::wasm_host::WasmHost;
use anyhow::Result;
use jamsocket::{JamsocketContext, JamsocketServiceFactory};
use std::{path::Path, sync::Arc};
use wasmtime::{Engine, Module};

/// Loads and caches a WebAssembly module such that a [WasmHost] instance can be
/// created from it.
///
/// This struct is cheaply cloneable, so it can be used to create multiple instances
/// of the same module.
#[derive(Clone)]
pub struct WasmHostFactory {
    engine: Arc<Engine>,
    module: Arc<Module>,
}

impl<T: JamsocketContext> JamsocketServiceFactory<T> for WasmHostFactory {
    type Service = WasmHost;

    fn build(&self, room_id: &str, context: T) -> Option<Self::Service> {
        let result = WasmHost::new(
            room_id,
            self.module.as_ref(),
            self.engine.as_ref(),
            &Arc::new(context),
        );

        match result {
            Ok(r) => Some(r),
            Err(e) => {
                log::error!("Could not build a WasmHost, got: {:?}", &e);
                None
            }
        }
    }
}

impl WasmHostFactory {
    pub fn new<P>(wasm_file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let engine = Engine::default();
        log::info!("Loading WebAssembly module {:?}", wasm_file.as_ref());
        let module = Module::from_file(&engine, wasm_file)?;

        Ok(WasmHostFactory {
            engine: Arc::new(engine),
            module: Arc::new(module),
        })
    }

    pub fn new_with_shared_module(engine: Arc<Engine>, module: Arc<Module>) -> Self {
        WasmHostFactory {
            engine,
            module
        }
    }
}
