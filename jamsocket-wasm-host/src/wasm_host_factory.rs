use crate::{process_module::load_module_bytes, wasm_host::WasmHost};
use anyhow::Result;
use jamsocket::{JamsocketContext, JamsocketServiceFactory};
use std::sync::Arc;
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

    fn build(&self, room_id: &str, context: T) -> Self::Service {
        WasmHost::new(
            room_id,
            self.module.as_ref(),
            self.engine.as_ref(),
            Arc::new(context),
        )
        .unwrap()
    }
}

impl WasmHostFactory {
    pub fn new(wasm_file: &str, preprocess: bool) -> Result<Self> {
        let engine = Engine::default();
        log::info!("Loading WebAssembly module {}", &wasm_file);

        let bytes = load_module_bytes(wasm_file, preprocess)?;

        let module = Module::from_binary(&engine, &bytes)?;

        Ok(WasmHostFactory {
            engine: Arc::new(engine),
            module: Arc::new(module),
        })
    }
}
