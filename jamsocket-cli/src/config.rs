use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};
use serde::Deserialize;

/// Represents a `jamsocket.toml` file, used to configure
/// a Jamsocket server.
#[derive(Deserialize, Debug, Default)]
pub struct JamsocketConfig {
    /// Directory to serve static files from.
    ///
    /// If this is provided, the server will attempt to serve HTTP requests
    /// relative to this directory, if they do not match other paths.
    pub static_files: Option<String>,

    /// Optional configuration for building a WebAssembly module for the
    /// client.
    ///
    /// This allows you to use `jamsocket dev` to build both the server- and
    /// client-side code from the same workspace in one command.
    pub client: Option<ClientConfig>,

    /// Configuration for building the WebAssembly module to serve.
    pub service: ServiceConfig,
}

/// Configuration for generating a client-side WebAssembly module.
#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    /// The name of the package to build.
    ///
    /// Must be discoverable by cargo from the directory that `jamsocket`
    /// is run from (i.e. `cargo build -p <package>` should succeed)
    pub package: String,
    pub optimization_level: Option<String>,
}

/// Configuration for generating and serving a Jamsocket service module.
#[derive(Deserialize, Debug, Default)]
pub struct ServiceConfig {
    /// The name of the package to build.
    ///
    /// Must be discoverable by cargo from the directory that `jamsocket`
    /// is run from (i.e. `cargo build -p <package>` should succeed)
    ///
    /// If this is empty, builds the package we are in (i.e. the package that
    /// `cargo build` builds.)
    pub package: Option<String>,

    /// The strategy used for generating room IDs.
    #[serde(default)]
    pub room_strategy: RoomIdStrategy,

    /// Specifies how empty rooms are handled.
    #[serde(default)]
    pub shutdown_policy: ServiceShutdownPolicy,
}
