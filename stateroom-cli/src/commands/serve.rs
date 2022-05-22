use crate::cli_opts::ServeCommand;
use stateroom_server::Server;
use stateroom_stdio::StdioProcessService;
use stateroom_wasm_host::WasmHost;
use std::{ffi::OsStr, path::Path, time::Duration};

pub fn serve(serve_opts: ServeCommand) -> anyhow::Result<()> {
    let ServeCommand {
        module,
        port,
        heartbeat_interval,
        heartbeat_timeout,
    } = serve_opts;

    let path = Path::new(&module);
    let ext = path
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
        let host_factory = WasmHost::new(&module)?;
        server_settings.serve(host_factory).map_err(|e| e.into())
    } else if path.is_file() {
        // Assume that module represents a system process.
        let host_factory = StdioProcessService::new(&module);
        server_settings.serve(host_factory).map_err(|e| e.into())
    } else if path.is_dir() {
        let server_module = path.join("server.wasm");

        if !server_module.exists() {
            return Err(anyhow::anyhow!("Expected server.wasm"));
        }

        let static_dir = path.join("static");

        let static_dir = if static_dir.exists() {
            Some(static_dir.to_str().unwrap().to_string())
        } else {
            None
        };

        let host_factory = WasmHost::new(&server_module)?;

        server_settings
            .with_static_path(static_dir)
            .serve(host_factory)
            .map_err(|e| e.into())
    } else {
        Err(anyhow::anyhow!("Expected a file or directory."))
    }
}
