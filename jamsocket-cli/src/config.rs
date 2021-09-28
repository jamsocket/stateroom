use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use anyhow::anyhow;
use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};
use serde::{Deserialize, Serialize};

const CONFIG_LOCATION_ENV_VAR: &str = "JAMSOCKET_CONFIG";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GlobalConfig {
    pub token: Option<String>,
}

pub struct GlobalConfigHandle {
    pub config: GlobalConfig,
    pub location: PathBuf,
    pub exists: bool,
}

impl GlobalConfigHandle {
    pub fn new() -> anyhow::Result<Self> {
        let conf_path = if let Ok(path) = std::env::var(CONFIG_LOCATION_ENV_VAR) {
            PathBuf::from(&path)
        } else {
            let home = dirs::home_dir().ok_or_else(||
                anyhow!("Home directory not found and {} not set; can't decide where to put the global.toml config file.",
                CONFIG_LOCATION_ENV_VAR))?;
            PathBuf::from(&home)
                .join(".config")
                .join("jamsocket")
                .join("global.toml")
        };

        let (config, exists) = if conf_path.exists() {
            let conf_toml = read_to_string(&conf_path)?;
            (toml::from_str(&conf_toml)?, true)
        } else {
            (GlobalConfig::default(), false)
        };

        Ok(GlobalConfigHandle {
            config,
            location: conf_path,
            exists,
        })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.location.parent() {
            create_dir_all(parent)?;
        }

        let conf_toml = toml::to_string_pretty(&self.config)?;
        std::fs::write(&self.location, &conf_toml).map_err(|e| e.into())
    }
}

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
