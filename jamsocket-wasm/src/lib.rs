pub use jamsocket_wasm_macro::jamsocket_wasm;
pub use wasm_host::WasmHost;
pub use wasm_host_factory::WasmHostFactory;

mod wasm_host;
mod wasm_host_factory;

use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum WasmRuntimeError {
    CouldNotImportMemory,
    CouldNotImportGlobal,
    InvalidApiVersion,
    InvalidProtocolVersion,
}

impl Display for WasmRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for WasmRuntimeError {
    fn description(&self) -> &str {
        match self {
            Self::CouldNotImportMemory => "Could not import memory from wasm instance.",
            Self::CouldNotImportGlobal => "Could not read global variable from wasm instance.",
            Self::InvalidApiVersion => {
                "WebAssembly module has an incompatible Jamsocket API version."
            }
            Self::InvalidProtocolVersion => {
                "WebAssembly module has an incompatible Jamsocket protocol version."
            }
        }
    }
}
