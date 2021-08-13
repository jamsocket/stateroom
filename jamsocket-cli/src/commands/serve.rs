use std::time::Duration;

use crate::cli_opts::ServeCommand;
use jamsocket_server::Server;
use jamsocket_wasm_host::WasmHostFactory;

pub fn serve(serve_opts: ServeCommand) -> anyhow::Result<()> {
    let ServeCommand {
        module,
        port,
        rooms,
        heartbeat_interval,
        heartbeat_timeout,
        shutdown_policy,
    } = serve_opts;

    let host_factory = WasmHostFactory::new(&module)?;
    let server_settings = Server {
        heartbeat_interval: Duration::from_secs(heartbeat_interval),
        heartbeat_timeout: Duration::from_secs(heartbeat_timeout),
        port,
        room_id_strategy: rooms,
        shutdown_policy,
        ..Server::default()
    };

    server_settings.serve(host_factory).map_err(|e| e.into())
}
