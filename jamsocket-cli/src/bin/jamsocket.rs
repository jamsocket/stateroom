//! # `jamsocket-cli`: a command-line interface to Jamsocket

use clap::Clap;
use jamsocket_cli::cli_opts::{Opts, SubCommand};
use jamsocket_cli::{dev, login, serve};
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::default()
        .add_directive("jamsocket_cli=info".parse()?)
        .add_directive("jamsocket_wasm_host=info".parse()?)
        .add_directive("jamsocket_server=info".parse()?);

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Serve(serve_opts) => serve(serve_opts),
        SubCommand::Dev => dev(),
        SubCommand::Login(login_opts) => login(login_opts),
    }
}
