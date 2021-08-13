use crate::config::JamsocketConfig;
use anyhow::{anyhow, Result};
use cargo_metadata::Message;
use jamsocket_server::Server;
use jamsocket_wasm_host::WasmHostFactory;
use std::{
    fs::read_to_string,
    path::PathBuf,
    process::{Command, Stdio},
};
use wasm_bindgen_cli_support::Bindgen;

fn locate_config() -> anyhow::Result<JamsocketConfig> {
    if let Ok(r) = read_to_string("jamsocket.toml") {
        log::info!("Loading config from file (jamsocket.toml).");
        toml::from_str(&r).map_err(|e| e.into())
    } else {
        log::info!("Didn't find a jamsocket.toml file in current directory, using default.");
        Ok(JamsocketConfig::default())
    }
}

fn run_cargo_build_command(
    package: &Option<String>,
    target: &str,
    release: bool,
) -> Result<PathBuf> {
    let mut build_command = Command::new("cargo");
    build_command.stdout(Stdio::piped());
    build_command.arg("build");
    build_command.args(["--message-format", "json-render-diagnostics"]);

    if let Some(package) = package {
        // If package is None, build the package we are in.
        build_command.args(["--package", package]);
    }
    build_command.args(["--target", target]);

    if release {
        build_command.arg("--release");
    }

    let mut build_command = build_command.spawn()?;
    let reader = std::io::BufReader::new(
        build_command
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Could not read stdout stream."))?,
    );

    let mut found_wasm_modules = Vec::new();

    for message in cargo_metadata::Message::parse_stream(reader) {
        match message {
            // TODO: handle error when toolchain is not installed, and retry after
            // attempting to install toolchain.
            Ok(Message::CompilerArtifact(artifact)) => {
                for filename in artifact.filenames {
                    if filename
                        .extension()
                        .map_or(false, |ext| ext.to_ascii_lowercase() == "wasm")
                    {
                        found_wasm_modules.push(filename);
                    }
                }
            }
            Ok(Message::BuildFinished(finished)) => {
                if !finished.success {
                    return Err(anyhow!("Build error."));
                }
            }
            Err(e) => return Err(anyhow!("Unknown error during build: {:?}.", e)),
            _ => (),
        }
    }

    build_command
        .wait()
        .map_err(|e| anyhow!("Encountered OS error running build subprocess: {:?}", e))?;

    let result = match found_wasm_modules.as_slice() {
        [] => return Err(anyhow!("No .wasm files emitted by build.")),
        [a] => a,
        _ => return Err(anyhow!("Multiple .wasm files emitted by build.")),
    };

    Ok(result.into())
}

pub fn dev() -> anyhow::Result<()> {
    let config = locate_config()?; // TODO: default to a configuration if file not found.

    log::info!("Building service");
    let service_wasm = run_cargo_build_command(&config.service.package, "wasm32-wasi", true)?;

    let host_factory = WasmHostFactory::new(service_wasm)?;

    let client_path = if let Some(client_config) = config.client {
        log::info!("Building client");
        let client_wasm_path =
            run_cargo_build_command(&Some(client_config.package), "wasm32-unknown-unknown", true)
                .expect("Error building client.");

        Bindgen::new()
            .input_path(client_wasm_path)
            .web(true)?
            .emit_start(false)
            .generate("client-pkg")?;

        // TODO: run wasm-opt
        Some("client-pkg".to_string())
    } else {
        None
    };

    Server::default()
        .with_static_path(config.static_files)
        .with_client_path(client_path)
        .with_room_id_strategy(config.service.room_strategy)
        .serve(host_factory)
        .map_err(|e| e.into())
}
