use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    /// Run a dev server to host a given Stateroom module.
    Serve(ServeCommand),

    Build,
    Dev {
        #[clap(default_value="8080")]
        port: u32,
    },
}

#[derive(Parser)]
pub struct LoginCommand {
    #[clap(short, long)]
    pub token: Option<String>,

    #[clap(short, long)]
    pub clear: bool,
}

#[derive(Parser)]
pub struct ServeCommand {
    /// The module (.wasm file) to serve.
    pub module: String,

    /// The port to serve on.
    #[clap(short, long, default_value = "8080")]
    pub port: u32,

    /// The time interval (in seconds) between WebSocket heartbeat pings.
    #[clap(short = 'i', long, default_value = "30")]
    pub heartbeat_interval: u64,

    /// The duration of time without hearing from a client before it is
    /// assumed to be disconnected.
    #[clap(short = 't', long, default_value = "120")]
    pub heartbeat_timeout: u64,
}
