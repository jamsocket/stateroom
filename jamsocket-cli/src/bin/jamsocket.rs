//! # `jamsocket-cli`: a command-line interface to Jamsocket

use clap::Clap;
use jamsocket_cli::cli_opts::{Opts, SubCommand};
use jamsocket_cli::{dev, serve};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Serve(serve_opts) => serve(serve_opts),
        SubCommand::Dev => dev(),
    }
}
