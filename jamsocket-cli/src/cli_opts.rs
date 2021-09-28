use clap::Clap;
use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    /// Run a dev server to host a given Jamsocket module.
    Serve(ServeCommand),

    Login(LoginCommand),

    Dev,
}

#[derive(Clap)]
pub struct LoginCommand {
    #[clap(short, long)]
    pub token: Option<String>,

    #[clap(short, long)]
    pub clear: bool,
}

#[derive(Clap)]
pub struct ServeCommand {
    /// The module (.wasm file) to serve.
    pub module: String,

    /// The port to serve on.
    #[clap(short, long, default_value = "8080")]
    pub port: u32,

    /// The strategy for assigning new room IDs.
    #[clap(short, long, default_value = "implicit")]
    pub rooms: RoomIdStrategy,

    /// The time interval (in seconds) between WebSocket heartbeat pings.
    #[clap(short = 'i', long, default_value = "30")]
    pub heartbeat_interval: u64,

    /// The duration of time without hearing from a client before it is
    /// assumed to be disconnected.
    #[clap(short = 't', long, default_value = "120")]
    pub heartbeat_timeout: u64,

    #[clap(short, long, default_value = "300sec")]
    pub shutdown_policy: ServiceShutdownPolicy,
}
