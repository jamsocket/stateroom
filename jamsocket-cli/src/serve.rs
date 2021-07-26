use crate::cli_opts::ServeCommand;
use jamsocket_server::do_serve;
use jamsocket_wasm_host::WasmHostFactory;

pub fn serve(serve_opts: ServeCommand) -> std::io::Result<()> {
    let ServeCommand {
        module,
        port,
        rooms,
    } = serve_opts;

    let host_factory = WasmHostFactory::new(&module);

    do_serve(host_factory, rooms, port)
}
