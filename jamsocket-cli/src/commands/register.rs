use crate::config::GlobalConfigHandle;
use colored::Colorize;
use jamsocket_api::JamsocketApi;
use std::fs::read_to_string;
use toml_edit::{value, Document};

const JAMSOCKET_TOML: &str = "jamsocket.toml";
const SERVICE_ID: &str = "service_id";

pub fn register() -> anyhow::Result<()> {
    let global_config = GlobalConfigHandle::new()?;
    let doc_str = match read_to_string(JAMSOCKET_TOML) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "".to_string(),
        Err(e) => return Err(e.into()),
    };
    let mut doc = doc_str.parse::<Document>()?;

    if doc[SERVICE_ID].is_str() {
        println!(
            "{}",
            "jamsocket.toml file already has a service_id, not overwriting"
                .red()
                .bold()
        );
        return Ok(());
    }

    let token = if let Some(token) = global_config.config.token {
        token
    } else {
        println!("Need to be logged in.");
        return Ok(());
    };

    let service_id = JamsocketApi::new(&token).new_service()?;

    doc[SERVICE_ID] = value(&service_id);
    std::fs::write(JAMSOCKET_TOML, doc.to_string())?;

    println!(
        "Created new service {} and wrote to jamsocket.toml.",
        service_id.green().bold()
    );

    Ok(())
}
