use std::path::Path;

use deno_core::ModuleSpecifier;

use super::errors::RiftResult;

pub fn specifier_from_file_path(path: &Path) -> RiftResult<ModuleSpecifier> {
    ModuleSpecifier::from_file_path(path)
        .map_err(|_| anyhow::anyhow!("Failed to create ModuleSpecifier from file path"))
}
