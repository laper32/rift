use crate::manifest::converter::PackageKind;
use std::collections::HashMap;

/// Virtual package (Workspace or Folder)
/// These are organizational packages that can have children
#[derive(Debug, Clone)]
pub struct VirtualPackage {
    pub name: String,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    // Script fields - resolved through inheritance
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    // Other metadata
    pub others: HashMap<String, toml::Value>,
    // Parent package for inheritance resolution
    pub parent: Option<Box<VirtualPackage>>,
}

/// Real package (Project or Target)
/// These are actual build units
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    // Script fields - resolved through inheritance
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    // Metadata
    pub others: HashMap<String, toml::Value>,
    // Parent package for inheritance resolution
    pub parent: Option<Box<VirtualPackage>>,
}

/// Rift plugin package
#[derive(Debug, Clone)]
pub struct RiftPackage {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    // Script fields - plugins don't inherit from parent
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    // Metadata
    pub others: HashMap<String, toml::Value>,
}

/// Runtime package variant
#[derive(Debug, Clone)]
pub enum MaybePackage {
    Workspace(VirtualPackage),
    Folder(VirtualPackage),
    Project(Package),
    Target(Package),
    Plugin(RiftPackage),
}

impl VirtualPackage {
    /// Create a workspace from PackageKind::Workspace manifest
    pub fn from_workspace_manifest(
        manifest: &crate::manifest::r#virtual::WorkspaceManifest,
        parent: Option<Box<VirtualPackage>>,
    ) -> Self {
        VirtualPackage {
            name: manifest.name.clone(),
            members: manifest.members.clone(),
            exclude: manifest.exclude.clone(),
            plugins: manifest.plugins.clone(),
            configure: manifest.configure.clone(),
            dependencies: manifest.dependencies.clone(),
            tasks: manifest.tasks.clone(),
            others: manifest.others.clone(),
            parent,
        }
    }

    /// Create a folder from PackageKind::Folder manifest
    pub fn from_folder_manifest(
        manifest: &crate::manifest::r#virtual::FolderManifest,
        parent: Option<Box<VirtualPackage>>,
    ) -> Self {
        VirtualPackage {
            name: manifest.name.clone(),
            members: manifest.members.clone(),
            exclude: manifest.exclude.clone(),
            plugins: manifest.plugins.clone(),
            configure: manifest.configure.clone(),
            dependencies: manifest.dependencies.clone(),
            tasks: manifest.tasks.clone(),
            others: manifest.others.clone(),
            parent,
        }
    }

    /// Resolve inherited scripts from parent
    /// Returns (plugins, configure, dependencies) with inheritance applied
    pub fn resolve_inherited_scripts(&self) -> (Option<String>, Option<String>, Option<String>) {
        match &self.parent {
            Some(parent) => {
                let (parent_plugins, parent_configure, parent_dependencies) =
                    parent.resolve_inherited_scripts();
                crate::manifest::converter::PackageKind::resolve_all_scripts(
                    self.plugins.as_deref(),
                    self.configure.as_deref(),
                    self.dependencies.as_deref(),
                    parent_plugins.as_deref(),
                    parent_configure.as_deref(),
                    parent_dependencies.as_deref(),
                )
            }
            None => {
                // Root of inheritance chain
                (
                    self.plugins.clone(),
                    self.configure.clone(),
                    self.dependencies.clone(),
                )
            }
        }
    }
}

impl Package {
    /// Create a project from PackageKind::Project manifest with parent context
    pub fn from_project_manifest(
        manifest: &crate::manifest::real::ProjectManifest,
        parent: Option<Box<VirtualPackage>>,
    ) -> Self {
        Package {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            authors: manifest.authors.clone(),
            description: manifest.description.clone(),
            plugins: manifest.plugins.clone(),
            configure: manifest.configure.clone(),
            dependencies: manifest.dependencies.clone(),
            tasks: manifest.tasks.clone(),
            others: manifest.others.clone(),
            parent,
        }
    }

    /// Create a target from PackageKind::Target manifest with parent context
    pub fn from_target_manifest(
        manifest: &crate::manifest::real::TargetManifest,
        parent: Option<Box<VirtualPackage>>,
    ) -> Self {
        Package {
            name: manifest.name.clone(),
            version: manifest.target_type.clone(),
            authors: vec![],
            description: None,
            plugins: manifest.plugins.clone(),
            configure: manifest.configure.clone(),
            dependencies: manifest.dependencies.clone(),
            tasks: manifest.tasks.clone(),
            others: manifest.others.clone(),
            parent,
        }
    }

    /// Resolve inherited scripts from parent
    /// Returns (plugins, configure, dependencies) with inheritance applied
    pub fn resolve_inherited_scripts(&self) -> (Option<String>, Option<String>, Option<String>) {
        match &self.parent {
            Some(parent) => {
                let (parent_plugins, parent_configure, parent_dependencies) =
                    parent.resolve_inherited_scripts();
                crate::manifest::converter::PackageKind::resolve_all_scripts(
                    self.plugins.as_deref(),
                    self.configure.as_deref(),
                    self.dependencies.as_deref(),
                    parent_plugins.as_deref(),
                    parent_configure.as_deref(),
                    parent_dependencies.as_deref(),
                )
            }
            None => {
                // Root of inheritance chain (no parent)
                (
                    self.plugins.clone(),
                    self.configure.clone(),
                    self.dependencies.clone(),
                )
            }
        }
    }
}

impl RiftPackage {
    /// Create a plugin from PackageKind::Plugin manifest
    pub fn from_plugin_manifest(manifest: &crate::manifest::rift::PluginManifest) -> Self {
        RiftPackage {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            authors: manifest.authors.clone(),
            description: manifest.description.clone(),
            configure: manifest.configure.clone(),
            dependencies: manifest.dependencies.clone(),
            tasks: manifest.tasks.clone(),
            others: manifest.others.clone(),
        }
    }
}

impl MaybePackage {
    /// Get package name
    pub fn name(&self) -> &str {
        match self {
            Self::Workspace(pkg) => &pkg.name,
            Self::Folder(pkg) => &pkg.name,
            Self::Project(pkg) => &pkg.name,
            Self::Target(pkg) => &pkg.name,
            Self::Plugin(pkg) => &pkg.name,
        }
    }

    /// Get parent package name (if any)
    pub fn parent_name(&self) -> Option<&str> {
        match self {
            Self::Workspace(pkg) => pkg.parent.as_ref().map(|p| p.name.as_str()),
            Self::Folder(pkg) => pkg.parent.as_ref().map(|p| p.name.as_str()),
            Self::Project(pkg) => pkg.parent.as_ref().map(|p| p.name.as_str()),
            Self::Target(pkg) => pkg.parent.as_ref().map(|p| p.name.as_str()),
            Self::Plugin(_) => None, // Plugins don't have parents
        }
    }

    /// Convert from PackageKind manifest with optional parent context
    pub fn from_package_kind(kind: PackageKind, parent: Option<Box<VirtualPackage>>) -> Self {
        match kind {
            PackageKind::Workspace(manifest) => {
                // Workspace is root, no parent
                Self::Workspace(VirtualPackage::from_workspace_manifest(&manifest, None))
            }
            PackageKind::Folder(manifest) => {
                Self::Folder(VirtualPackage::from_folder_manifest(&manifest, parent))
            }
            PackageKind::Project(manifest) => {
                Self::Project(Package::from_project_manifest(&manifest, parent))
            }
            PackageKind::Target(manifest) => {
                Self::Target(Package::from_target_manifest(&manifest, parent))
            }
            PackageKind::Plugin(manifest) => {
                Self::Plugin(RiftPackage::from_plugin_manifest(&manifest))
            }
        }
    }
}
