use std::time::Duration;

use crate::cli_opts::ServeCommand;
use jamsocket_server::{do_serve, ServerSettings, ServiceShutdownPolicy};
use jamsocket_wasm_host::WasmHostFactory;

pub fn serve(serve_opts: ServeCommand) -> std::io::Result<()> {
    let ServeCommand {
        module,
        port,
        rooms,
        heartbeat_interval,
        heartbeat_timeout,
    } = serve_opts;

    let host_factory = WasmHostFactory::new(&module);
    let server_settings = ServerSettings {
        heartbeat_interval: Duration::from_secs(heartbeat_interval),
        heartbeat_timeout: Duration::from_secs(heartbeat_timeout),
        port,
        room_id_strategy: rooms,
        shutdown_policy: ServiceShutdownPolicy::Immediate,
    };

    do_serve(host_factory, server_settings)
}
