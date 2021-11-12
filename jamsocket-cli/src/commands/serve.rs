use std::{ffi::OsStr, path::Path, time::Duration};

use crate::cli_opts::ServeCommand;
use jamsocket_server::Server;
use jamsocket_stdio::StdioProcessServiceFactory;
use jamsocket_wasm_host::WasmHostFactory;

pub fn serve(serve_opts: ServeCommand) -> anyhow::Result<()> {
    let ServeCommand {
        module,
        port,
        heartbeat_interval,
        heartbeat_timeout,
    } = serve_opts;

    let ext = Path::new(&module)
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_ascii_lowercase);

    let server_settings = Server {
        heartbeat_interval: Duration::from_secs(heartbeat_interval),
        heartbeat_timeout: Duration::from_secs(heartbeat_timeout),
        port,
        ..Server::default()
    };

    if let Some("wasm" | "wat") = ext.as_deref() {
        let host_factory = WasmHostFactory::new(&module)?;
        server_settings.serve(host_factory).map_err(|e| e.into())
    } else {
        // Assume that module represents a system process.
        let host_factory = StdioProcessServiceFactory::new(&module);
        server_settings.serve(host_factory).map_err(|e| e.into())
    }
}
