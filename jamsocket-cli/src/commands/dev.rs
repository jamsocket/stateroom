use crate::config::JamsocketConfig;
use convert_case::{Case, Casing};
use core::panic;
use jamsocket_server::Server;
use jamsocket_wasm_host::WasmHostFactory;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::Command,
};
use wasm_bindgen_cli_support::Bindgen;

fn locate_config() -> anyhow::Result<JamsocketConfig> {
    let r = read_to_string("jamsocket.toml")?;
    toml::from_str(&r).map_err(|e| e.into())
}

fn run_cargo_build_command(package: &str, target: &str, release: bool) -> std::io::Result<PathBuf> {
    let mut build_command = Command::new("cargo");
    build_command.arg("build");
    build_command.args(["--package", package]);
    build_command.args(["--target", target]);

    if release {
        build_command.arg("--release");
    }

    let status = build_command.status()?;
    if !status.success() {
        panic!() // TODO: handle this
    }

    let subdir = if release { "release" } else { "debug" };

    let package_norm = format!("{}.wasm", package.to_case(Case::Snake));

    let expected_path = Path::new("target")
        .join(&target)
        .join(subdir)
        .join(package_norm);

    if !expected_path.exists() {
        panic!(
            "Expected file {} to exist after build, but it does not.",
            expected_path.to_str().unwrap()
        );
    }

    Ok(expected_path)
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
            run_cargo_build_command(&client_config.package, "wasm32-unknown-unknown", true)
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
