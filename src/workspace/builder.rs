use anyhow::{Result, anyhow, bail};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::manifest::converter::convert_toml_to_manifest;
use crate::schema::TomlManifest;
use crate::workspace::package::{MaybePackage, VirtualPackage};

/// Workspace builder - scans and builds the package tree
pub struct WorkspaceBuilder {
    /// Root directory of the workspace
    root: PathBuf,
    /// All discovered packages by name
    packages: HashMap<String, MaybePackage>,
    /// Package path to name mapping
    path_to_name: HashMap<PathBuf, String>,
}

impl WorkspaceBuilder {
    /// Create a new workspace builder
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            packages: HashMap::new(),
            path_to_name: HashMap::new(),
        }
    }

    /// Build the workspace by scanning from root
    pub fn build(
        mut self,
    ) -> Result<(
        MaybePackage,
        HashMap<String, MaybePackage>,
        HashMap<String, PathBuf>,
    )> {
        // Find and parse root manifest
        let root_manifest_path = self.find_root_manifest()?;

        // Parse root manifest
        let root_package = self.load_and_register_package(&root_manifest_path, None)?;

        // Recursively scan all virtual packages (Workspace/Folder)
        self.scan_virtual_packages(&root_package)?;

        Ok((
            root_package,
            self.packages,
            self.path_to_name
                .clone()
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect(),
        ))
    }

    /// Find the root Rift.toml manifest
    fn find_root_manifest(&self) -> Result<PathBuf> {
        let manifest = self.root.join("Rift.toml");
        if !manifest.exists() {
            bail!(
                "No Rift.toml found in root directory: {}",
                self.root.display()
            );
        }
        Ok(manifest)
    }

    /// Load and register a package from its manifest path
    fn load_and_register_package(
        &mut self,
        manifest_path: &Path,
        parent: Option<&VirtualPackage>,
    ) -> Result<MaybePackage> {
        eprintln!("DEBUG: load_and_register_package: path={:?}", manifest_path.display());

        // Normalize path
        let manifest_path = manifest_path.canonicalize()?;

        // Check if already loaded
        if let Some(name) = self.path_to_name.get(&manifest_path) {
            return Ok(self
                .packages
                .get(name)
                .ok_or_else(|| anyhow!("Package name not found: {}", name))?
                .clone());
        }

        // Parse TOML
        let toml: TomlManifest = toml::from_str(&fs::read_to_string(&manifest_path)?)?;

        // Convert to PackageKind
        let package_kind = convert_toml_to_manifest(&toml)?;

        // Create MaybePackage
        let parent_ref = parent.map(|p| Box::new(p.clone()));
        let package = MaybePackage::from_package_kind(package_kind, parent_ref);

        // Register package
        let name = package.name().to_string();
        self.path_to_name.insert(manifest_path, name.clone());

        // Handle name conflicts
        if self.packages.contains_key(&name) {
            bail!("Duplicate package name: {}", name);
        }

        self.packages.insert(name.clone(), package.clone());

        Ok(package)
    }

    /// Recursively scan virtual packages (Workspace/Folder) for children
    fn scan_virtual_packages(&mut self, root_package: &MaybePackage) -> Result<()> {
        match root_package {
            MaybePackage::Workspace(ws) => {
                self.scan_members(&ws.name, &ws.members, &ws.exclude, Some(ws))?;
            }
            MaybePackage::Folder(folder) => {
                self.scan_members(&folder.name, &folder.members, &folder.exclude, Some(folder))?;
            }
            MaybePackage::Project(_) | MaybePackage::Target(_) | MaybePackage::Plugin(_) => {
                // Leaf nodes, no children to scan
            }
        }
        Ok(())
    }

    /// Scan members and exclude patterns
    fn scan_members(
        &mut self,
        _parent_name: &str,
        members: &[String],
        exclude: &[String],
        parent_package: Option<&VirtualPackage>,
    ) -> Result<()> {
        let parent_dir = match parent_package {
            Some(pkg) => {
                // Find the manifest path for this package
                let manifest_path = self.find_manifest_path_by_name(&pkg.name)?;
                manifest_path
                    .parent()
                    .ok_or_else(|| anyhow!("Package manifest has no parent: {}", pkg.name))?
                    .to_path_buf()
            }
            None => self.root.clone(),
        };

        // Collect all candidate paths from members patterns
        let mut candidates = Vec::new();
        for pattern in members {
            let expanded = self.expand_glob_pattern(&parent_dir, pattern)?;
            candidates.extend(expanded);
        }

        // Filter out excluded paths
        let filtered: Vec<_> = candidates
            .into_iter()
            .filter(|path| !self.is_excluded(path, exclude, &parent_dir))
            .collect();

        // Load each child package
        for child_path in filtered {
            eprintln!("DEBUG: Loading child package: {:?}", child_path.display());
            match self.load_and_register_package(&child_path, parent_package) {
                Ok(child_package) => {
                    eprintln!("DEBUG: Loaded package: {}", child_package.name());
                    // Recursively scan if child is also a virtual package
                    self.scan_virtual_packages(&child_package)?;
                }
                Err(e) => {
                    eprintln!("DEBUG: Failed to load package {:?}: {}", child_path.display(), e);
                }
            }
        }

        Ok(())
    }

    /// Expand a glob pattern to find matching Rift.toml files
    fn expand_glob_pattern(&self, base_dir: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
        let mut results = Vec::new();

        let pattern_path = base_dir.join(pattern);

        // Debug output
        eprintln!("DEBUG builder: expand_glob_pattern: base_dir={}, pattern={}", base_dir.display(), pattern);

        // Check if it's a direct path (no wildcards)
        if !pattern.contains('*') && !pattern.contains('?') {
            if pattern_path.is_dir() {
                // It's a directory, look for Rift.toml inside
                let manifest = pattern_path.join("Rift.toml");
                eprintln!("DEBUG builder: pattern_path is_dir, checking manifest at: {}", manifest.display());
                if manifest.exists() {
                    eprintln!("DEBUG builder: FOUND manifest at: {}", manifest.display());
                    results.push(manifest);
                } else {
                    eprintln!("DEBUG builder: manifest NOT found at: {}", manifest.display());
                    // Scan subdirectories for Rift.toml
                    if let Ok(entries) = fs::read_dir(&pattern_path) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_dir() {
                                let manifest = path.join("Rift.toml");
                                if manifest.exists() {
                                    results.push(manifest);
                                }
                            }
                        }
                    }
                }
            } else if pattern_path.exists() {
                results.push(pattern_path);
            }
        } else {
            // Use glob for pattern matching
            let pattern_str = pattern_path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid glob pattern path"))?;

            // Use glob crate for pattern matching
            for entry in glob::glob(pattern_str)?.flatten() {
                if entry.is_dir() {
                    let manifest = entry.join("Rift.toml");
                    if manifest.exists() {
                        results.push(manifest);
                    }
                } else if entry.ends_with("Rift.toml") {
                    results.push(entry);
                }
            }
        }

        Ok(results)
    }

    /// Check if a path matches any exclude pattern
    fn is_excluded(&self, path: &Path, exclude: &[String], base_dir: &Path) -> bool {
        for pattern in exclude {
            let pattern_path = base_dir.join(pattern);
            let pattern_str = pattern_path.to_str().unwrap_or("");

            // Direct match
            if path == pattern_path {
                return true;
            }

            // Parent directory match
            if path.starts_with(&pattern_path) {
                return true;
            }

            // Glob pattern match
            if pattern.contains('*') || pattern.contains('?') {
                if let Ok(matches) = glob::glob(pattern_str) {
                    for m in matches.flatten() {
                        if path.starts_with(&m) || path == m {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Find manifest path by package name
    fn find_manifest_path_by_name(&self, name: &str) -> Result<PathBuf> {
        self.path_to_name
            .iter()
            .find(|(_, n)| *n == name)
            .map(|(path, _)| path.clone())
            .ok_or_else(|| anyhow!("Package not found: {}", name))
    }

    /// Get all discovered packages
    pub fn packages(&self) -> &HashMap<String, MaybePackage> {
        &self.packages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    fn create_test_workspace() -> Result<PathBuf> {
        let temp = std::env::temp_dir().join("rift_test_workspace");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp)?;

        // Create root workspace
        let root_toml = r#"
[workspace]
name = "test-workspace"
members = ["pkg1", "pkg2"]
"#;
        File::create(temp.join("Rift.toml"))?.write_all(root_toml.as_bytes())?;

        // Create pkg1
        let pkg1_dir = temp.join("pkg1");
        fs::create_dir_all(&pkg1_dir)?;
        let pkg1_toml = r#"
[project]
name = "pkg1"
version = "0.1.0"
authors = []
"#;
        File::create(pkg1_dir.join("Rift.toml"))?.write_all(pkg1_toml.as_bytes())?;

        // Create pkg2
        let pkg2_dir = temp.join("pkg2");
        fs::create_dir_all(&pkg2_dir)?;
        let pkg2_toml = r#"
[project]
name = "pkg2"
version = "0.1.0"
authors = []
"#;
        File::create(pkg2_dir.join("Rift.toml"))?.write_all(pkg2_toml.as_bytes())?;

        Ok(temp)
    }

    #[test]
    fn test_workspace_builder() {
        let root = create_test_workspace().unwrap();
        let builder = WorkspaceBuilder::new(root);
        let (workspace, packages, _manifests) = builder.build().unwrap();

        match workspace {
            MaybePackage::Workspace(ws) => {
                assert_eq!(ws.name, "test-workspace");
                assert_eq!(ws.members.len(), 2);
                assert_eq!(packages.len(), 3); // workspace + 2 projects
            }
            _ => panic!("Expected workspace"),
        }
    }

    #[test]
    fn test_expand_glob_directories() {
        let temp = std::env::temp_dir().join("rift_glob_test");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        // Create some directories
        fs::create_dir_all(temp.join("dir1")).unwrap();
        fs::create_dir_all(temp.join("dir2")).unwrap();

        // Create manifests in them
        File::create(temp.join("dir1/Rift.toml"))
            .unwrap()
            .write_all(b"[project]\nname = \"dir1\"\nversion = \"0.1.0\"\nauthors = []")
            .unwrap();
        File::create(temp.join("dir2/Rift.toml"))
            .unwrap()
            .write_all(b"[project]\nname = \"dir2\"\nversion = \"0.1.0\"\nauthors = []")
            .unwrap();

        let builder = WorkspaceBuilder::new(temp.clone());

        // Test direct directory pattern
        let results = builder.expand_glob_pattern(&temp, "dir1").unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("dir1/Rift.toml"));

        let _ = fs::remove_dir_all(temp);
    }
}
