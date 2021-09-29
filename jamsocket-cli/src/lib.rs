mod api;
pub mod cli_opts;
mod commands;
mod config;

pub use commands::deploy::deploy;
pub use commands::dev::dev;
pub use commands::login::login;
pub use commands::serve::serve;

pub(crate) const API_BASE: &str = "https://beta.jamsocket.com/";
pub(crate) const WS_BASE: &str = "wss://beta.jamsocket.com/";
