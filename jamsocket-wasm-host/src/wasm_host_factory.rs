use crate::wasm_host::WasmHost;
use jamsocket::{JamsocketContext, JamsocketServiceBuilder};
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

impl<T: JamsocketContext + Send + Sync + 'static> JamsocketServiceBuilder<T> for WasmHostFactory {
    type Service = WasmHost;

    fn build(self, room_id: &str, context: T) -> Self::Service {
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
    pub fn new(wasm_file: &str) -> Self {
        let engine = Engine::default();
        log::info!("Loading WebAssembly module {}", &wasm_file);
        let module = Module::from_file(&engine, wasm_file).unwrap();

        WasmHostFactory {
            engine: Arc::new(engine),
            module: Arc::new(module),
        }
    }
}
