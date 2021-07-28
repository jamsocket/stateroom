//! # `jamsocket-cli`: a command-line interface to Jamsocket

mod cli_opts;
mod serve;

use clap::Clap;
use cli_opts::{Opts, SubCommand};
use env_logger::Builder;
use serve::serve;
use std::io::Result;

fn main() -> Result<()> {
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
