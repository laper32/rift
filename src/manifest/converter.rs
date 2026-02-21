use anyhow::{Result, anyhow};

use crate::manifest::{
    real::{ProjectManifest, TargetManifest},
    rift::PluginManifest,
    r#virtual::{FolderManifest, WorkspaceManifest},
};
use crate::schema::TomlManifest;

/// Detects package kind and converts TOML schema to Manifest enum.
/// Detection order: Workspace → Folder → Project → Target → Plugin
pub fn convert_toml_to_manifest(toml: &TomlManifest) -> Result<PackageKind> {
    // 1. Workspace detection
    if let Some(workspace) = &toml.workspace {
        if toml.folder.is_some()
            || toml.project.is_some()
            || toml.target.is_some()
            || toml.plugin.is_some()
        {
            return Err(anyhow!(
                "error[conflicting-sections]: [workspace] cannot coexist with [folder], [project], [target], or [plugin]"
            ));
        }

        let name = workspace.name.clone().unwrap_or_default();
        let members = workspace.members.clone().unwrap_or_default();
        let exclude = workspace.exclude.clone().unwrap_or_default();

        return Ok(PackageKind::Workspace(WorkspaceManifest {
            name,
            members,
            exclude,
            plugins: workspace.plugins.clone(),
            configure: workspace.configure.clone(),
            dependencies: workspace.dependencies.clone(),
            tasks: workspace.tasks.clone(),
            others: workspace.others.clone(),
        }));
    }

    // 2. Folder detection
    if let Some(folder) = &toml.folder {
        if toml.workspace.is_some()
            || toml.project.is_some()
            || toml.target.is_some()
            || toml.plugin.is_some()
        {
            return Err(anyhow!(
                "error[conflicting-sections]: [folder] cannot coexist with [workspace], [project], [target], or [plugin]"
            ));
        }

        let name = folder.name.clone().unwrap_or_default();
        let members = folder.members.clone().unwrap_or_default();
        let exclude = folder.exclude.clone().unwrap_or_default();

        // Folder now supports script inheritance fields
        return Ok(PackageKind::Folder(FolderManifest {
            name,
            members,
            exclude,
            plugins: folder.plugins.clone(),
            configure: folder.configure.clone(),
            dependencies: folder.dependencies.clone(),
            tasks: folder.tasks.clone(),
            others: folder.others.clone(),
        }));
    }

    // 3. Project detection
    if let Some(project) = &toml.project {
        if toml.plugin.is_some() {
            return Err(anyhow!(
                "error[conflicting-sections]: [project] cannot coexist with [plugin]"
            ));
        }

        // Validate mutual exclusion: target vs members/exclude
        let target_manifest = if let Some(target) = &toml.target {
            if project.members.is_some() || project.exclude.is_some() {
                return Err(anyhow!(
                    "error[invalid-config]: [project.members] and [project.exclude] cannot occur when [target] field exists"
                ));
            }

            // Note: When target exists at same level as project, target's scripts are ignored
            // Only project-level scripts are used (as per C# version design)
            Some(TargetManifest {
                name: target.name.clone(),
                target_type: target.target_type.clone(),
                plugins: None,      // Ignored when target is within project
                configure: None,    // Ignored when target is within project
                dependencies: None, // Ignored when target is within project
                tasks: None,        // Ignored when target is within project
                others: target.others.clone(),
            })
        } else {
            None
        };

        let project_manifest = ProjectManifest {
            name: project.name.clone(),
            authors: project.authors.clone(),
            version: project.version.clone(),
            description: project.description.clone(),
            plugins: project.plugins.clone(),
            configure: project.configure.clone(),
            dependencies: project.dependencies.clone(),
            tasks: project.tasks.clone(),
            target: target_manifest,
            members: project.members.clone(),
            exclude: project.exclude.clone(),
            others: project.others.clone(),
        };

        return Ok(PackageKind::Project(project_manifest));
    }

    // 4. Target detection (standalone target without project)
    if let Some(target) = &toml.target {
        if toml.plugin.is_some() {
            return Err(anyhow!(
                "error[conflicting-sections]: [target] cannot coexist with [plugin]"
            ));
        }

        let target_manifest = TargetManifest {
            name: target.name.clone(),
            target_type: target.target_type.clone(),
            plugins: target.plugins.clone(),
            configure: target.configure.clone(),
            dependencies: target.dependencies.clone(),
            tasks: target.tasks.clone(),
            others: target.others.clone(),
        };

        return Ok(PackageKind::Target(target_manifest));
    }

    // 5. Plugin detection
    if let Some(plugin) = &toml.plugin {
        let plugin_manifest = PluginManifest {
            name: plugin.name.clone(),
            authors: plugin.authors.clone(),
            version: plugin.version.clone(),
            description: plugin.description.clone(),
            configure: plugin.configure.clone(),
            dependencies: plugin.dependencies.clone(),
            tasks: plugin.tasks.clone(),
            others: plugin.others.clone(),
        };

        return Ok(PackageKind::Plugin(plugin_manifest));
    }

    Err(anyhow!(
        "error[no-package-type]: No valid package section found ([workspace], [folder], [project], [target], or [plugin] required)"
    ))
}

/// Represents the detected package kind with appropriate manifest.
#[derive(Debug)]
pub enum PackageKind {
    Workspace(WorkspaceManifest),
    Folder(FolderManifest),
    Project(ProjectManifest),
    Target(TargetManifest),
    Plugin(PluginManifest),
}

impl PackageKind {
    /// Get the package name.
    pub fn name(&self) -> &str {
        match self {
            Self::Workspace(m) => &m.name,
            Self::Folder(m) => &m.name,
            Self::Project(m) => &m.name,
            Self::Target(m) => &m.name,
            Self::Plugin(m) => &m.name,
        }
    }

    /// Check if this is a script-capable package (can inherit scripts).
    pub fn is_script_capable(&self) -> bool {
        !matches!(self, Self::Target(_))
    }

    /// Resolve a script field with inheritance logic.
    /// If `own_script` is Some, use it; otherwise fall back to `inherited_script`.
    /// Applies to: plugins, configure, dependencies
    pub fn resolve_script(
        own_script: Option<&str>,
        inherited_script: Option<&str>,
    ) -> Option<String> {
        own_script.or(inherited_script).map(|s| s.to_string())
    }

    /// Resolve all three script fields at once.
    /// Returns (plugins, configure, dependencies) with inheritance applied.
    pub fn resolve_all_scripts(
        own_plugins: Option<&str>,
        own_configure: Option<&str>,
        own_dependencies: Option<&str>,
        inherited_plugins: Option<&str>,
        inherited_configure: Option<&str>,
        inherited_dependencies: Option<&str>,
    ) -> (Option<String>, Option<String>, Option<String>) {
        (
            Self::resolve_script(own_plugins, inherited_plugins),
            Self::resolve_script(own_configure, inherited_configure),
            Self::resolve_script(own_dependencies, inherited_dependencies),
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::schema::{
        real::TomlProject,
        r#virtual::{TomlFolder, TomlWorkspace},
    };

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_workspace_detection() {
        let toml = TomlManifest {
            workspace: Some(TomlWorkspace {
                name: Some("my-workspace".to_string()),
                members: Some(vec!["pkg1".to_string(), "pkg2".to_string()]),
                exclude: Some(vec!["test".to_string()]),
                plugins: None,
                configure: None,
                dependencies: None,
                others: HashMap::new(),
                tasks: None,
            }),
            folder: None,
            project: None,
            target: None,
            plugin: None,
        };

        let result = convert_toml_to_manifest(&toml);
        assert!(result.is_ok());

        if let Ok(PackageKind::Workspace(manifest)) = result {
            assert_eq!(manifest.name, "my-workspace");
            assert_eq!(manifest.members.len(), 2);
            assert_eq!(manifest.exclude.len(), 1);
        } else {
            panic!("Expected Workspace variant");
        }
    }

    #[test]
    fn test_folder_detection() {
        let toml = TomlManifest {
            workspace: None,
            folder: Some(TomlFolder {
                name: Some("subfolder".to_string()),
                members: None,
                exclude: None,
                plugins: Some("plugins.ts".to_string()),
                configure: Some("configure.ts".to_string()),
                dependencies: Some("deps.ts".to_string()),
                others: HashMap::new(),
                tasks: None,
            }),
            project: None,
            target: None,
            plugin: None,
        };

        let result = convert_toml_to_manifest(&toml);
        assert!(result.is_ok());

        if let Ok(PackageKind::Folder(manifest)) = result {
            assert_eq!(manifest.name, "subfolder");
            assert_eq!(manifest.plugins, Some("plugins.ts".to_string()));
            assert_eq!(manifest.configure, Some("configure.ts".to_string()));
            assert_eq!(manifest.dependencies, Some("deps.ts".to_string()));
        } else {
            panic!("Expected Folder variant");
        }
    }

    #[test]
    fn test_project_detection() {
        let toml = TomlManifest {
            workspace: None,
            folder: None,
            project: Some(TomlProject {
                name: "my-project".to_string(),
                authors: vec![],
                version: "1.0.0".to_string(),
                description: None,
                plugins: None,
                configure: None,
                dependencies: None,
                members: None,
                exclude: None,
                others: HashMap::new(),
                tasks: None,
            }),
            target: None,
            plugin: None,
        };

        let result = convert_toml_to_manifest(&toml);
        assert!(result.is_ok());

        if let Ok(PackageKind::Project(manifest)) = result {
            assert_eq!(manifest.name, "my-project");
            assert_eq!(manifest.version, "1.0.0");
        } else {
            panic!("Expected Project variant");
        }
    }

    #[test]
    fn test_workspace_folder_conflict() {
        let toml = TomlManifest {
            workspace: Some(TomlWorkspace {
                name: Some("workspace".to_string()),
                members: None,
                exclude: None,
                plugins: None,
                configure: None,
                dependencies: None,
                others: HashMap::new(),
                tasks: None,
            }),
            folder: Some(TomlFolder {
                name: Some("folder".to_string()),
                members: None,
                exclude: None,
                plugins: None,
                configure: None,
                dependencies: None,
                others: HashMap::new(),
                tasks: None,
            }),
            project: None,
            target: None,
            plugin: None,
        };

        let result = convert_toml_to_manifest(&toml);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("conflicting-sections")
        );
    }

    #[test]
    fn test_script_inheritance_resolution() {
        // Child inherits from parent when child is None
        let parent_configure = Some("src/configure.ts");
        let child_configure = None;

        let resolved = PackageKind::resolve_script(child_configure, parent_configure);
        assert_eq!(resolved, Some("src/configure.ts".to_string()));

        // Child script overrides parent
        let parent_configure = Some("src/configure.ts");
        let child_configure = Some("subfolder/configure.ts");

        let resolved = PackageKind::resolve_script(child_configure, parent_configure);
        assert_eq!(resolved, Some("subfolder/configure.ts".to_string()));

        // Neither has script
        let resolved: Option<String> = PackageKind::resolve_script(None, None);
        assert_eq!(resolved, None);
    }

    #[test]
    fn test_all_scripts_inheritance() {
        // Parent (workspace) has all three scripts
        let parent_plugins = Some("workspace/plugins.ts");
        let parent_configure = Some("workspace/configure.ts");
        let parent_dependencies = Some("workspace/deps.ts");

        // Child (folder) overrides only plugins
        let child_plugins = Some("linux/plugins.ts");
        let child_configure = None;
        let child_dependencies = None;

        let (plugins, configure, dependencies) = PackageKind::resolve_all_scripts(
            child_plugins,
            child_configure,
            child_dependencies,
            parent_plugins,
            parent_configure,
            parent_dependencies,
        );

        // Child plugins overrides parent
        assert_eq!(plugins, Some("linux/plugins.ts".to_string()));
        // Child inherits parent's configure/dependencies
        assert_eq!(configure, Some("workspace/configure.ts".to_string()));
        assert_eq!(dependencies, Some("workspace/deps.ts".to_string()));
    }

    #[test]
    fn test_plugins_inheritance_different_platforms() {
        // Workspace defines base plugins
        let workspace_plugins = Some("plugins/common.ts");

        // Linux folder needs different plugins
        let linux_folder_plugins = Some("plugins/linux.ts");

        // Windows folder needs different plugins
        let windows_folder_plugins = Some("plugins/windows.ts");

        // Linux folder resolution
        let resolved_linux = PackageKind::resolve_script(linux_folder_plugins, workspace_plugins);
        assert_eq!(resolved_linux, Some("plugins/linux.ts".to_string()));

        // Windows folder resolution
        let resolved_windows =
            PackageKind::resolve_script(windows_folder_plugins, workspace_plugins);
        assert_eq!(resolved_windows, Some("plugins/windows.ts".to_string()));
    }

    #[test]
    fn test_manifest_to_package_inheritance() {
        use crate::workspace::package::MaybePackage;

        // Create workspace manifest
        let toml = TomlManifest {
            workspace: Some(TomlWorkspace {
                name: Some("root-workspace".to_string()),
                members: Some(vec!["project1".to_string()]),
                exclude: None,
                plugins: Some("workspace/plugins.ts".to_string()),
                configure: Some("workspace/configure.ts".to_string()),
                dependencies: Some("workspace/deps.ts".to_string()),
                others: HashMap::new(),
                tasks: None,
            }),
            folder: None,
            project: None,
            target: None,
            plugin: None,
        };

        // Convert TOML → PackageKind (Manifest)
        let package_kind = convert_toml_to_manifest(&toml);
        assert!(package_kind.is_ok());

        // Convert PackageKind → MaybePackage (Runtime)
        let package = MaybePackage::from_package_kind(package_kind.unwrap(), None);

        // Verify workspace package
        match package {
            MaybePackage::Workspace(ws) => {
                assert_eq!(ws.name, "root-workspace");
                assert_eq!(ws.members.len(), 1);

                // Verify script fields
                let (plugins, configure, dependencies) = ws.resolve_inherited_scripts();
                assert_eq!(plugins, Some("workspace/plugins.ts".to_string()));
                assert_eq!(configure, Some("workspace/configure.ts".to_string()));
                assert_eq!(dependencies, Some("workspace/deps.ts".to_string()));
            }
            _ => panic!("Expected Workspace variant"),
        }
    }

    #[test]
    fn test_folder_inherits_from_workspace() {
        use crate::workspace::package::VirtualPackage;

        // Create workspace as parent
        let workspace_manifest = WorkspaceManifest {
            name: "workspace".to_string(),
            members: vec![],
            exclude: vec![],
            plugins: Some("plugins/common.ts".to_string()),
            configure: Some("configure/common.ts".to_string()),
            dependencies: Some("deps/common.ts".to_string()),
            others: HashMap::new(),
            tasks: None,
        };
        let workspace = VirtualPackage::from_workspace_manifest(&workspace_manifest, None);

        // Create folder as child (overrides plugins only)
        let folder_manifest = FolderManifest {
            name: "linux-folder".to_string(),
            members: vec![],
            exclude: vec![],
            plugins: Some("plugins/linux.ts".to_string()), // Override
            configure: None,                               // Inherit
            dependencies: None,                            // Inherit
            others: HashMap::new(),
            tasks: None,
        };

        let folder =
            VirtualPackage::from_folder_manifest(&folder_manifest, Some(Box::new(workspace)));

        // Resolve inheritance
        let (plugins, configure, dependencies) = folder.resolve_inherited_scripts();

        // Folder plugins override workspace
        assert_eq!(plugins, Some("plugins/linux.ts".to_string()));
        // Folder inherits workspace's configure
        assert_eq!(configure, Some("configure/common.ts".to_string()));
        // Folder inherits workspace's dependencies
        assert_eq!(dependencies, Some("deps/common.ts".to_string()));
    }
}
