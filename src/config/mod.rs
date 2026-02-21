//! Rift configuration
//!
//! Handles reading `.rift/config.toml` for build system settings.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Rift configuration from .rift/config.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiftConfig {
    /// Index configuration
    #[serde(default)]
    pub index: IndexConfig,
}

/// Index sources configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexConfig {
    /// Index source URLs
    #[serde(default)]
    pub sources: Vec<String>,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            sources: vec![
                "https://github.com/rift-lang/index".to_string(),
            ],
        }
    }
}

impl Default for RiftConfig {
    fn default() -> Self {
        Self {
            index: IndexConfig::default(),
        }
    }
}

/// Find and read .rift/config.toml
///
/// Searches upward from the given path until finding .rift/config.toml
pub fn load_config(start_dir: &Path) -> Result<Option<RiftConfig>> {
    let config_path = find_config(start_dir)?;

    match config_path {
        Some(path) => {
            let content = fs::read_to_string(&path)?;
            let config: RiftConfig = toml::from_str(&content)
                .map_err(|e| anyhow!("Failed to parse config.toml: {}", e))?;
            Ok(Some(config))
        }
        None => Ok(None),
    }
}

/// Find .rift/config.toml by searching upward from start_dir
fn find_config(start_dir: &Path) -> Result<Option<PathBuf>> {
    let mut current = Some(start_dir);

    while let Some(dir) = current {
        let config_file = dir.join(".rift").join("config.toml");
        if config_file.exists() {
            return Ok(Some(config_file));
        }

        // Move to parent directory
        current = dir.parent();
    }

    Ok(None)
}

/// Get the default config path (in user's home directory)
pub fn default_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(home.join(".rift").join("config.toml"))
}

/// Load config from workspace directory, falling back to default
pub fn load_config_or_default(workspace_dir: &Path) -> RiftConfig {
    load_config(workspace_dir)
        .ok()
        .flatten()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RiftConfig::default();
        assert_eq!(config.index.sources.len(), 1);
        assert_eq!(config.index.sources[0], "https://github.com/rift-lang/index");
    }

    #[test]
    fn test_index_config_default() {
        let config = IndexConfig::default();
        assert_eq!(config.sources.len(), 1);
    }
}
