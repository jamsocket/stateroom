use crate::build_util::{do_build, locate_config};
use jamsocket_server::Server;
use jamsocket_wasm_host::WasmHostFactory;

pub fn dev(port: u32) -> anyhow::Result<()> {
    let config = locate_config()?; // TODO: default to a configuration if file not found.

    let build_result = do_build(&config)?;
    let host_factory = WasmHostFactory::new(build_result.server_wasm)?;

    Server::default()
        .with_port(port)
        .with_static_path(config.static_files)
        .with_client_path(build_result.client_wasm)
        .serve(host_factory)
        .map_err(|e| e.into())
}
