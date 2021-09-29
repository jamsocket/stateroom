use crate::{API_BASE, WS_BASE, api::upload, cli_opts::DeployCommand, commands::dev::{locate_config, run_cargo_build_command}, config::GlobalConfigHandle};
use anyhow::anyhow;
use std::{fs::File, io::Read};
use colored::Colorize;

pub fn deploy(deploy_opts: DeployCommand) -> anyhow::Result<()> {
    let service_config = locate_config()?; // TODO: default to a configuration if file not found.
    let global_config = GlobalConfigHandle::new()?;

    tracing::info!("Building service");
    let service_wasm =
        run_cargo_build_command(&service_config.service.package, "wasm32-wasi", true)?;

    let mut module: Vec<u8> = Vec::new();
    File::open(service_wasm)?.read_to_end(&mut module)?;

    let service_id = deploy_opts.service_id;

    let token = global_config.config.token.ok_or(anyhow!(
        "Use `jamsocket login` first to install jamsocket credentials."
    ))?;
    let result = upload(&token, &service_id, &module)?;

    let new_room_url = format!("{}service/{}/{}/new_room", API_BASE, result.service, result.module);
    let ws_url = format!("{}service/{}/{}/ws/<room id>", WS_BASE, result.service, result.module);

    println!("Module uploaded successfully.\n\nNew room URL:\n\n{} {}\n\nWebsocket URL:\n\n{}",
        "POST".blue().bold(),
        new_room_url.yellow().bold(),
        ws_url.yellow().bold(),
    );

    Ok(())
}
