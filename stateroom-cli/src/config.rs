use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GlobalConfig {
    pub token: Option<String>,
}

/// Represents a `stateroom.toml` file, used to configure
/// a Stateroom server.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StateroomConfig {
    /// Directory to serve static files from.
    ///
    /// If this is provided, the server will attempt to serve HTTP requests
    /// relative to this directory, if they do not match other paths.
    pub static_files: Option<String>,

    /// A unique ID for the service used for deployment.
    pub service_id: Option<String>,

    /// Optional configuration for building a WebAssembly module for the
    /// client.
    ///
    /// This allows you to use `stateroom dev` to build both the server- and
    /// client-side code from the same workspace in one command.
    pub client: Option<ClientConfig>,

    /// Configuration for building the WebAssembly module to serve.
    #[serde(default)]
    pub service: ServiceConfig,
}

/// Configuration for generating a client-side WebAssembly module.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    /// The name of the package to build.
    ///
    /// Must be discoverable by cargo from the directory that `stateroom`
    /// is run from (i.e. `cargo build -p <package>` should succeed)
    pub package: String,
    pub optimization_level: Option<String>,
}

/// Configuration for generating and serving a Stateroom service module.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceConfig {
    /// The name of the package to build.
    ///
    /// Must be discoverable by cargo from the directory that `stateroom`
    /// is run from (i.e. `cargo build -p <package>` should succeed)
    ///
    /// If this is empty, builds the package we are in (i.e. the package that
    /// `cargo build` builds.)
    pub package: Option<String>,
}
