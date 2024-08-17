use crate::config::StateroomConfig;
use anyhow::{anyhow, Result};
use cargo_metadata::Message;
use std::{
    fs::read_to_string,
    path::PathBuf,
    process::{Command, Stdio},
};
use wasm_bindgen_cli_support::Bindgen;

pub fn locate_config() -> anyhow::Result<StateroomConfig> {
    if let Ok(r) = read_to_string("stateroom.toml") {
        tracing::info!("Loading config from file (stateroom.toml)");
        toml::from_str(&r).map_err(|e| e.into())
    } else {
        tracing::info!("Didn't find a stateroom.toml file in current directory, using default");
        Ok(StateroomConfig::default())
    }
}

pub fn run_cargo_build_command(
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
                        if filename.parent().map_or(false, |p| p.ends_with("deps")) {
                            continue;
                        }

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
        files => {
            return Err(anyhow!(
                "Multiple .wasm files emitted by build ({:?}).",
                files
            ))
        }
    };

    Ok(result.into())
}

pub struct BuildResult {
    pub server_wasm: String,
    pub client_wasm: Option<String>,
}

pub fn do_build(config: &StateroomConfig) -> Result<BuildResult> {
    tracing::info!("Building service");
    let server_wasm = run_cargo_build_command(&config.service.package, "wasm32-wasi", true)?;

    let client_wasm = if let Some(client_config) = &config.client {
        tracing::info!("Building client");
        let client_wasm_path = run_cargo_build_command(
            &Some(client_config.package.to_string()),
            "wasm32-unknown-unknown",
            true,
        )
        .expect("Error building client.");

        Bindgen::new()
            .input_path(client_wasm_path)
            .web(true)?
            .emit_start(false)
            .typescript(true)
            .generate("client-pkg")?;

        // TODO: run wasm-opt
        Some("client-pkg".to_string())
    } else {
        None
    };

    Ok(BuildResult {
        client_wasm,
        server_wasm: server_wasm.to_str().unwrap().to_string(),
    })
}
