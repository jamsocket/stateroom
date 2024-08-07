use crate::wasm_host::WasmHost;
use anyhow::Result;
use stateroom::{StateroomContext, StateroomServiceFactory};
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

impl StateroomServiceFactory for WasmHostFactory {
    type Service = WasmHost;
    type Error = anyhow::Error;

    fn build(
        &self,
        room_id: &str,
        context: Arc<impl StateroomContext>,
    ) -> Result<Self::Service, Self::Error> {
        WasmHost::new(room_id, self.module.as_ref(), self.engine.as_ref(), context)
    }
}

impl WasmHostFactory {
    pub fn new<P>(wasm_file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let engine = Engine::default();
        tracing::info!(wasm_file=?wasm_file.as_ref(), "Loading WebAssembly module");
        let module = Module::from_file(&engine, wasm_file)?;
        tracing::info!("WebAssembly module loaded");

        Ok(WasmHostFactory {
            engine: Arc::new(engine),
            module: Arc::new(module),
        })
    }

    #[must_use]
    pub fn new_with_shared_module(engine: Arc<Engine>, module: Arc<Module>) -> Self {
        WasmHostFactory { engine, module }
    }
}
