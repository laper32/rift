//! Package index for plugin registry
//!
//! Follows Cargo's registry design with Git-based index and JSON Lines format.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufRead, BufReader};

/// Index configuration from config.json
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexConfig {
    /// Download URL template
    /// Variables: {plugin} - plugin name, {version} - version
    #[serde(default = "default_download_template")]
    pub dl: String,
}

fn default_download_template() -> String {
    "https://cdn.rift-lang.org/plugins/{plugin}/{version}/download".to_string()
}

/// A package version entry from the index (one JSON line)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageEntry {
    /// Package name
    pub name: String,
    /// Version string
    pub vers: String,
    /// Download URL (optional, if not using template)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Checksum (sha256:...)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cksum: Option<String>,
}

/// Cached index data
#[derive(Debug, Clone)]
pub struct CachedIndex {
    /// Index URL
    pub url: String,
    /// Config from index
    pub config: IndexConfig,
    /// Packages indexed by name -> version -> entry
    pub packages: HashMap<String, HashMap<String, PackageEntry>>,
}

impl CachedIndex {
    /// Create a new cached index
    pub fn new(url: String, config: IndexConfig) -> Self {
        Self {
            url,
            config,
            packages: HashMap::new(),
        }
    }

    /// Find a package entry by name and version
    pub fn find(&self, name: &str, version: &str) -> Option<&PackageEntry> {
        self.packages.get(name)?.get(version)
    }

    /// Get all versions for a package
    pub fn versions(&self, name: &str) -> Option<Vec<&PackageEntry>> {
        self.packages.get(name)
            .map(|versions| versions.values().collect())
    }

    /// Add a package entry
    pub fn add_entry(&mut self, entry: PackageEntry) {
        self.packages
            .entry(entry.name.clone())
            .or_insert_with(HashMap::new)
            .insert(entry.vers.clone(), entry);
    }
}

/// Get the cache directory for indexes
pub fn index_cache_dir() -> Result<PathBuf> {
    let cache = crate::git::cache_dir()?;
    Ok(cache.parent().unwrap().join("index"))
}

/// Get the cache path for a specific index
pub fn index_cache_path(url: &str) -> Result<PathBuf> {
    let cache = index_cache_dir()?;

    // Create a deterministic name from the URL
    let url_without_proto = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let clean_name: String = url_without_proto
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '_' })
        .collect();

    // Use .json extension for the cached index
    Ok(cache.join(format!("{}.json", clean_name)))
}

/// Fetch and cache an index from a URL
pub fn fetch_index(url: &str) -> Result<CachedIndex> {
    let cache_path = index_cache_path(url)?;

    // Check if we have a cached index that's fresh enough (optional, for now always fetch)
    // For now, let's just always fetch to keep it simple
    // TODO: Add cache validation and TTL

    // If it's a git URL, we need to clone it
    if url.ends_with(".git") || url.contains("github.com") || url.contains("gitlab.com") {
        fetch_git_index(url, &cache_path)
    } else {
        fetch_http_index(url, &cache_path)
    }
}

/// Fetch index from a Git repository
fn fetch_git_index(_url: &str, _cache_path: &Path) -> Result<CachedIndex> {
    // For Git-based indexes (like Cargo), we would:
    // 1. Clone or fetch the repo
    // 2. Parse the index files
    //
    // This is more complex since we need to:
    // - Clone to a temporary location
    // - Parse the index structure
    // - Or use git to fetch specific files

    // For now, return an error - HTTP indexes are simpler
    Err(anyhow!("Git index fetching not yet implemented. Use HTTPS index URL."))
}

/// Fetch index from an HTTPS endpoint
fn fetch_http_index(url: &str, cache_path: &Path) -> Result<CachedIndex> {
    // For HTTP-based indexes, we expect:
    // - A config.json at the root with the download template
    // - Package files following the Cargo naming convention

    // Ensure cache directory exists
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Try to fetch config.json from the index URL
    let config_url = format!("{}/config.json", url.trim_end_matches('/'));
    let config: IndexConfig = match ureq::get(&config_url).call() {
        Ok(response) => response.into_json()?,
        Err(e) => {
            // If config.json doesn't exist, use default template
            IndexConfig {
                dl: url.trim_end_matches('/').to_string() + "/{plugin}/{version}/download",
            }
        }
    };

    let index = CachedIndex::new(url.to_string(), config);

    // Cache the config
    let config_json = serde_json::to_string_pretty(&index.config)?;
    fs::write(cache_path, config_json)?;

    Ok(index)
}

/// Query for a package in the index
pub fn query_package(
    index_url: &str,
    package_name: &str,
    version: &str,
) -> Result<PackageEntry> {
    let index = fetch_index(index_url)?;

    // Try to find the package in cached data
    if let Some(entry) = index.find(package_name, version) {
        return Ok(entry.clone());
    }

    // If not cached, try to fetch the package file from the index
    // This is useful for HTTP-based indexes
    if !index_url.ends_with(".git") {
        if let Ok(entry) = fetch_package_entry_from_http(&index, package_name, version) {
            return Ok(entry);
        }
    }

    // Package not found
    let available = index.versions(package_name)
        .map(|v| v.iter().map(|e| e.vers.clone()).collect::<Vec<_>>())
        .unwrap_or_default();

    let available_str = if available.is_empty() {
        "no versions available".to_string()
    } else {
        format!("available versions: {}", available.join(", "))
    };

    Err(anyhow!(
        "Package '{}' version '{}' not found in index '{}'. {}",
        package_name, version, index_url, available_str
    ))
}

/// Get the package file path for a given package name (Cargo-style)
///
/// Cargo uses a tiered directory structure based on package name length:
/// - 1 character: packages/{name}/
/// - 2 characters: packages/2-chars/{name}/
/// - 3 characters: packages/3-chars/{name}/
/// - 4+ characters: packages/{first2}/{second2}/{name}/
fn package_file_path(name: &str) -> String {
    let name_lower = name.to_lowercase();
    let chars: Vec<char> = name_lower.chars().collect();

    match chars.len() {
        0 => unreachable!(),
        1 => format!("packages/{}", name_lower),
        2 => format!("packages/2-chars/{}", name_lower),
        3 => format!("packages/3-chars/{}", name_lower),
        _ => {
            let first2: String = chars[0..2].iter().collect();
            let second2: String = chars[2..4].iter().collect();
            format!("packages/{}/{}", first2, second2)
        }
    }
}

/// Fetch a package entry from an HTTP index
fn fetch_package_entry_from_http(
    index: &CachedIndex,
    package_name: &str,
    version: &str,
) -> Result<PackageEntry> {
    let file_path = package_file_path(package_name);
    let url = format!(
        "{}/{}/{}",
        index.url.trim_end_matches('/'),
        file_path,
        package_name
    );

    match ureq::get(&url).call() {
        Ok(response) => {
            let text = response.into_string()?;
            let cursor = std::io::Cursor::new(text);
            let reader = BufReader::new(cursor);
            let entries = parse_package_file(reader)?;

            // Find the specific version
            for entry in entries {
                if entry.name == package_name && entry.vers == version {
                    return Ok(entry);
                }
            }

            Err(anyhow!("Version {} not found for package {}", version, package_name))
        }
        Err(ureq::Error::Status(code, _)) => {
            Err(anyhow!("HTTP error {} when fetching package file for {}", code, package_name))
        }
        Err(e) => Err(anyhow!("Failed to fetch package file for {}: {}", package_name, e))
    }
}

/// Parse a package file (JSON Lines format)
pub fn parse_package_file<R: BufRead>(reader: R) -> Result<Vec<PackageEntry>> {
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let entry: PackageEntry = serde_json::from_str(&line)?;
        entries.push(entry);
    }

    Ok(entries)
}

/// Load package entries from the index for a specific package
pub fn load_package_entries(
    index_url: &str,
    package_name: &str,
) -> Result<Vec<PackageEntry>> {
    // In a full implementation, this would:
    // 1. Determine the package file path (by name length, Cargo-style)
    // 2. Fetch the package file from the index
    // 3. Parse the JSON Lines format

    // For now, return empty since we haven't implemented the full fetching
    let index = fetch_index(index_url)?;
    let empty: Vec<&PackageEntry> = Vec::new();
    let result = match index.versions(package_name) {
        Some(v) => v.iter().map(|e| (*e).clone()).collect(),
        None => vec![],
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_entry() {
        let json = r#"{"name":"test","vers":"1.0.0","url":"https://example.com/test.tar.gz","cksum":"abc123"}"#;
        let entry: PackageEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.name, "test");
        assert_eq!(entry.vers, "1.0.0");
        assert_eq!(entry.url, Some("https://example.com/test.tar.gz".to_string()));
        assert_eq!(entry.cksum, Some("abc123".to_string()));
    }

    #[test]
    fn test_parse_package_file() {
        let json = r#"{"name":"test","vers":"1.0.0"}
{"name":"test","vers":"1.1.0"}
{"name":"other","vers":"1.0.0"}"#;

        let cursor = std::io::Cursor::new(json);
        let reader = BufReader::new(cursor);
        let entries = parse_package_file(reader).unwrap();

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, "test");
        assert_eq!(entries[0].vers, "1.0.0");
        assert_eq!(entries[1].name, "test");
        assert_eq!(entries[1].vers, "1.1.0");
        assert_eq!(entries[2].name, "other");
    }
}
