use crate::config::{GlobalConfigHandle, JamsocketConfig, ServiceConfig};
use colored::Colorize;
use jamsocket_api::JamsocketApi;
use jamsocket_server::{RoomIdStrategy, ServiceShutdownPolicy};
use std::{fs::File, path::Path};
use std::io::prelude::Write;

pub fn init() -> anyhow::Result<()> {
    if Path::new("jamsocket.toml").exists() {
        println!(
            "{}",
            "jamsocket.toml file already exists, not overwriting"
                .red()
                .bold()
        );
        return Ok(());
    }

    let global_config = GlobalConfigHandle::new()?;

    if let Some(token) = global_config.config.token {
        let service_id = JamsocketApi::new(&token).new_service()?;
        let service_config = JamsocketConfig {
            static_files: None,
            client: None,
            service: ServiceConfig {
                package: None,
                room_strategy: RoomIdStrategy::Explicit,
                shutdown_policy: ServiceShutdownPolicy::Immediate,
            },
            service_id: Some(service_id.clone()),
        };

        let mut file = File::create("jamsocket.toml")?;
        file.write_all(&toml::to_vec(&service_config)?)?;

        println!(
            "Created new service {} and wrote to jamsocket.toml.",
            service_id.green().bold()
        );
    } else {
        println!("Need to be logged in.")
    }

    Ok(())
}
