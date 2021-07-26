use crate::wasm_host::WasmHost;
use jamsocket::JamsocketContext;
use std::sync::Arc;
use wasmtime::{Engine, Module};

#[derive(Clone)]
pub struct WasmHostFactory {
    engine: Arc<Engine>,
    module: Arc<Module>,
}

impl WasmHostFactory {
    pub fn create_room(&self, wctx: impl JamsocketContext + Send + Sync + 'static) -> WasmHost {
        let engine = self.engine.clone();
        let module = self.module.clone();

        WasmHost::new(module.as_ref(), engine.as_ref(), Arc::new(wctx)).unwrap()
    }

    pub fn new(wasm_file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_file).unwrap();

        WasmHostFactory {
            engine: Arc::new(engine),
            module: Arc::new(module),
        }
    }
}
