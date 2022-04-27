//! # `stateroom-cli`: a command-line interface to Stateroom

use clap::Parser;
use stateroom_cli::cli_opts::{Opts, SubCommand};
use stateroom_cli::{build, dev, serve};
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::default()
        .add_directive("stateroom_cli=info".parse()?)
        .add_directive("stateroom_wasm_host=info".parse()?)
        .add_directive("stateroom_server=info".parse()?);

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Serve(serve_opts) => serve(serve_opts),
        SubCommand::Dev { port } => dev(port),
        SubCommand::Build => build(),
    }
}
