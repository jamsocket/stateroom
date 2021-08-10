use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct JamsocketConfig {
    pub static_files: Option<String>,
    pub client: Option<ClientConfig>,
    pub service: ServiceConfig,
}

#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    pub package: String,
    pub optimization_level: Option<String>,
    pub url_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ServiceConfig {
    pub package: String,
    pub url_path: String,

    #[serde(default)]
    pub room_strategy: RoomIdStrategy,

    #[serde(default)]
    pub shutdown_policy: ServiceShutdownPolicy,
}
