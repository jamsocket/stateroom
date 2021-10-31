use crate::{
    cli_opts::DeployCommand,
    commands::dev::{locate_config, run_cargo_build_command},
    config::GlobalConfigHandle,
};
use anyhow::anyhow;
use colored::Colorize;
use jamsocket_api::JamsocketApi;
use std::{fs::File, io::Read};

pub fn deploy(deploy_opts: DeployCommand) -> anyhow::Result<()> {
    let service_config = locate_config()?; // TODO: default to a configuration if file not found.
    let global_config = GlobalConfigHandle::new()?;

    tracing::info!("Building service");
    let service_wasm =
        run_cargo_build_command(&service_config.service.package, "wasm32-wasi", true)?;

    let mut module: Vec<u8> = Vec::new();
    File::open(service_wasm)?.read_to_end(&mut module)?;

    let service_id = if let Some(service_id) = deploy_opts.service_id {
        service_id
    } else if let Some(service_id) = service_config.service_id {
        service_id
    } else {
        println!("{}\n\n{}",
        "The service id must either be passed in the command line, or be present in the jamsocket.toml file.".red().bold(),
        "Use `jamsocket init` to create a new service id.".yellow()
    );
        return Ok(());
    };

    let token = global_config.config.token.ok_or(anyhow!(
        "Use `jamsocket login` first to install jamsocket credentials."
    ))?;
    let result = JamsocketApi::new(&token).upload(&service_id, &module)?;

    println!("{}", "Module uploaded successfully.".green().bold());

    if let Some(create_room_url) = result.create_room_url {
        println!(
            "To create a room, send a {} request to:\n\n{}",
            "POST".blue().bold(),
            create_room_url.yellow().bold(),
        );
    } else {
        println!(
            "To create a room, open a WebSocket connection to:\n\n{}",
            result.url_base.yellow().bold()
        );
    }

    Ok(())
}
