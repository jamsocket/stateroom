use crate::wasm_host::WasmHost;
use jamsocket::JamsocketContext;
use std::sync::Arc;
use wasmtime::{Engine, Module};

pub struct WasmHostFactory {
    engine: Engine,
    module: Module,
}

impl WasmHostFactory {
    pub fn create_room(&self, wctx: impl JamsocketContext + Send + Sync + 'static) -> WasmHost {
        WasmHost::new(&self.module, &self.engine, Arc::new(wctx)).unwrap()
    }

    pub fn new(wasm_file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_file).unwrap();

        WasmHostFactory {
            engine,
            module,
        }
    }
}
