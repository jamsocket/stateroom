use crate::commands::dev::{locate_config, run_cargo_build_command};

pub fn deploy() -> anyhow::Result<()> {
    let config = locate_config()?; // TODO: default to a configuration if file not found.

    tracing::info!("Building service");
    let _service_wasm = run_cargo_build_command(&config.service.package, "wasm32-wasi", true)?;

    todo!()
}
