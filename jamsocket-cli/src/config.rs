use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct JamsocketConfig {
    pub static_files: Option<String>,
    pub client: Option<ClientConfig>,
    pub service: ServiceConfig,
}

#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    pub package: String,
    pub optimization_level: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ServiceConfig {
    pub package: Option<String>,

    #[serde(default)]
    pub room_strategy: RoomIdStrategy,

    #[serde(default)]
    pub shutdown_policy: ServiceShutdownPolicy,
}
