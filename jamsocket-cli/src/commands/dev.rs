use crate::config::JamsocketConfig;
use cargo_metadata::Message;
use core::panic;
use jamsocket_server::Server;
use jamsocket_wasm_host::WasmHostFactory;
use std::{fs::read_to_string, path::PathBuf, process::{Command, Stdio}};
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
) -> std::io::Result<PathBuf> {
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
    let reader = std::io::BufReader::new(build_command.stdout.take().unwrap());

    let mut found_wasm_modules = Vec::new();

    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerArtifact(artifact) => {
                for filename in artifact.filenames {
                    // TODO: investigate why `.as_str()` is required. `Utf8Path` has
                    // a function called `ends_with()`, but it seems never to return
                    // `true` here.
                    if filename.as_str().ends_with(".wasm") {
                        found_wasm_modules.push(filename);
                    }
                }
            }
            Message::BuildFinished(finished) => {
                if !finished.success {
                    panic!("Build error.")
                }
            }
            _ => (),
        }
    }

    build_command.wait().expect("h1");

    let result = match &found_wasm_modules.as_slice() {
        &[] => panic!("No wasm files built."),
        &[a] => a,
        _ => panic!("Found more than one wasm file and got confused."),
    };

    Ok(result.into())
}

pub fn dev() -> anyhow::Result<()> {
    let config = locate_config()?; // TODO: default to a configuration if file not found.

    log::info!("Building service");
    let service_wasm = run_cargo_build_command(&config.service.package, "wasm32-wasi", true)
        .expect("Error building service.");

    let host_factory = WasmHostFactory::new(service_wasm.to_str().unwrap());

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
