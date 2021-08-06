//! # `jamsocket-cli`: a command-line interface to Jamsocket

use clap::Clap;
use env_logger::Builder;
use jamsocket_cli::cli_opts::{Opts, SubCommand};
use jamsocket_cli::serve;

fn main() -> anyhow::Result<()> {
    let mut builder = Builder::new();
    builder.filter(Some("jamsocket_server"), log::LevelFilter::Info);
    builder.filter(Some("jamsocket_wasm_host"), log::LevelFilter::Info);
    builder.init();

    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Serve(serve_opts) => serve(serve_opts),
        SubCommand::Validate(_validate_opts) => {
            unimplemented!()
        }
    }
}
