use crate::wasm_host::{EXPECTED_EXPORTS, EXPECTED_IMPORTS};
use anyhow::Result;
use std::{collections::HashSet, path::Path};
use walrus::Module;

fn process_module(mut module: Module) -> Result<Module> {
    log::info!("Preprocessing module.");
    let mut exports_to_delete = HashSet::new();
    for export in module.exports.iter() {
        if !EXPECTED_EXPORTS.contains(&export.name.as_str()) {
            exports_to_delete.insert(export.id());
        }
    }

    if exports_to_delete.is_empty() {
        log::info!("Scanned module, but didn't make any changes.");
        return Ok(module);
    }

    log::info!("Found {} exports to remove.", exports_to_delete.len());

    for id in exports_to_delete {
        module.exports.delete(id);
    }
    module.start = None;

    walrus::passes::gc::run(&mut module);

    let mut customs_to_delete = HashSet::new();
    for (id, custom) in module.customs.iter() {
        //log::info!("Custom section: {:?}", custom);
        customs_to_delete.insert(id);
    }

    for id in customs_to_delete {
        module.customs.delete(id);
    }

    for export in module.exports.iter() {
        log::info!("Export: {}", export.name);
    }

    let mut imports_to_delete = HashSet::new();
    for import in module.imports.iter() {
        if !EXPECTED_IMPORTS.contains(&import.name.as_str()) {
            imports_to_delete.insert(import.id());
        }
    }

    for id in imports_to_delete {
        module.imports.delete(id);
    }

    Ok(module)
}

pub fn load_module_bytes<P: AsRef<Path>>(file: P, preprocess: bool) -> Result<Vec<u8>> {
    if preprocess {
        let mut module = Module::from_file(file)?;
        module = process_module(module)?;
        Ok(module.emit_wasm())
    } else {
        std::fs::read(file).map_err(|d| d.into())
    }
}
