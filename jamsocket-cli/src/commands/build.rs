use crate::build_util::{do_build, locate_config};
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use std::{
    fs::{copy, create_dir, remove_dir_all},
    path::Path,
};

const OUTPUT_DIR: &str = "dist";
const STATIC_DIR: &str = "static";

pub fn build() -> anyhow::Result<()> {
    let config = locate_config()?; // TODO: default to a configuration if file not found.

    let build_result = do_build(&config)?;

    if Path::new(OUTPUT_DIR).exists() {
        remove_dir_all(OUTPUT_DIR).context("Couldn't delete dist directory.")?;
    }
    create_dir(OUTPUT_DIR).context("Couldn't create dist directory.")?;

    copy(
        build_result.server_wasm,
        Path::new(OUTPUT_DIR).join("server.wasm"),
    )
    .context("Couldn't copy server.wasm.")?;

    create_dir(Path::new(OUTPUT_DIR).join(STATIC_DIR))
            .context("Couldn't create empty static directory.")?;

    if let Some(static_dir) = config.static_files {
        fs_extra::dir::copy(static_dir, Path::new(OUTPUT_DIR).join(STATIC_DIR), &CopyOptions {
            content_only: true,
            ..CopyOptions::default()
        }).context("Couldn't copy static items.")?;
    }

    if let Some(client_wasm) = build_result.client_wasm {
        fs_extra::dir::copy(client_wasm, Path::new(OUTPUT_DIR).join(STATIC_DIR).join("client"), &CopyOptions {
            copy_inside: true,
            ..CopyOptions::default()
        }).context("Couldn't copy client wasm.")?;
    }

    Ok(())
}
