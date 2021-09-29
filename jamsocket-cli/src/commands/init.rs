use crate::config::GlobalConfigHandle;
use colored::Colorize;
use jamsocket_api::JamsocketApi;
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
        let service_toml = format!("service_id = {}\n", service_id);

        let mut file = File::create("jamsocket.toml")?;
        file.write_all(&service_toml.as_bytes())?;

        println!(
            "Created new service {} and wrote to jamsocket.toml.",
            service_id.green().bold()
        );
    } else {
        println!("Need to be logged in.")
    }

    Ok(())
}
