use serde::Serialize;

#[derive(Serialize)]
pub struct ConnectionInfo {
    pub active_connections: u32,
    pub seconds_inactive: u32,
    pub listening: bool,
}
