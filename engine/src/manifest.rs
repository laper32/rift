use std::path::{Path, PathBuf};

use crate::{errors::RiftResult, schema, workspace::MaybePackage};

pub const MANIFEST_IDENTIFIER: &str = "Rift.toml";

#[derive(Debug)]
pub enum EitherManifest {
    Real(Manifest),
    Virtual(VirtualManifest),
}

#[derive(Debug)]
pub struct WorkspaceManifest {
    /// 没啥说的
    pub members: Vec<String>,

    /// 没啥说的
    pub exclude: Option<Vec<String>>,

    /// 指向的是文件路径
    pub metadata: Option<String>,

    /// 指向的是文件路径
    pub plugins: Option<String>,

    /// 指向的是文件路径
    pub dependencies: Option<String>,
}

#[derive(Debug)]
pub struct FolderManifest {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ProjectManifest {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
    // 如果project和target同时存在，那么members和exclude将无法使用，就算里面写东西也会被忽略
    // 除此之外无限制。
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct TargetManifest {
    pub name: String,
    pub build_type: String,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
}

pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub dependencies: Option<String>,
}

/// 严格定义的话，Manifest是可以作为包管理的
/// 下面三个都可以上传到包管理去
#[derive(Debug)]
pub enum Manifest {
    Project(ProjectManifest),
    Target(TargetManifest),
}

/// 这两个严格来说只用来组织项目结构
#[derive(Debug)]
pub enum VirtualManifest {
    Workspace(WorkspaceManifest),
    Folder(FolderManifest),
}

pub fn find_root_manifest(current_path: &PathBuf) -> Option<PathBuf> {
    let parent_manifest_path = current_path.parent()?.join(MANIFEST_IDENTIFIER);
    if parent_manifest_path.exists() {
        Some(parent_manifest_path)
    } else {
        find_root_manifest(&parent_manifest_path.parent()?.to_path_buf())
    }
}

pub fn read_manifest(path: &Path) -> RiftResult<EitherManifest> {
    let manifest = (|| {
        let deserialized_toml = schema::load_manifest(&path.to_path_buf());
        match deserialized_toml {
            Ok(manifest) => {
                if manifest.workspace.is_some() {
                    if manifest.folder.is_some()
                        || manifest.project.is_some()
                        || manifest.target.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!(
                            "Workspace and Folder/Project/Target/Plugin can't be used together."
                        )
                    }
                    let workspace_manifest = manifest.workspace.unwrap();
                    return Ok(EitherManifest::Virtual(VirtualManifest::Workspace(
                        WorkspaceManifest {
                            members: workspace_manifest.members,
                            exclude: Some(workspace_manifest.exclude),
                            metadata: workspace_manifest.metadata,
                            plugins: workspace_manifest.plugins,
                            dependencies: workspace_manifest.dependencies,
                        },
                    )));
                } else if manifest.folder.is_some() {
                    if manifest.workspace.is_some()
                        || manifest.project.is_some()
                        || manifest.target.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!(
                            "Workspace and Folder/Project/Target/Plugin can't be used together."
                        )
                    }
                    let folder_manifest = manifest.folder.unwrap();
                    return Ok(EitherManifest::Virtual(VirtualManifest::Folder(
                        FolderManifest {
                            members: Some(folder_manifest.members),
                            exclude: Some(folder_manifest.exclude),
                        },
                    )));
                } else if manifest.project.is_some() {
                    let project_manifest = manifest.project.unwrap();
                    if manifest.workspace.is_some()
                        || manifest.folder.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!("Workspace and Folder/Project/Plugin can't be used together.")
                    }
                    let mut project_manifest_actual = ProjectManifest {
                        name: project_manifest.name,
                        authors: project_manifest.authors,
                        version: project_manifest.version,
                        description: project_manifest.description,
                        plugins: project_manifest.plugins,
                        dependencies: project_manifest.dependencies,
                        metadata: project_manifest.metadata,
                        members: Some(project_manifest.members),
                        exclude: Some(project_manifest.exclude),
                    };

                    if manifest.target.is_some() {
                        project_manifest_actual.members = None;
                        project_manifest_actual.exclude = None;
                    }

                    return Ok(EitherManifest::Real(Manifest::Project(
                        project_manifest_actual,
                    )));
                } else if manifest.target.is_some() {
                    let target_manifest = manifest.target.unwrap();
                    if manifest.workspace.is_some()
                        || manifest.folder.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!("Workspace and Folder/Target/Plugin can't be used together.")
                    }

                    return Ok(EitherManifest::Real(Manifest::Target(TargetManifest {
                        name: target_manifest.name,
                        build_type: target_manifest.build_type,
                        plugins: target_manifest.plugins,
                        dependencies: target_manifest.dependencies,
                        metadata: target_manifest.metadata,
                    })));
                } else {
                    anyhow::bail!("No any schema field found.")
                }
            }
            Err(e) => anyhow::bail!("error: {:?}", e),
        }
    })();

    Ok(manifest?)
}

#[cfg(test)]
mod test {
    use crate::{
        manifest::{find_root_manifest, read_manifest},
        util::get_cargo_project_root,
    };

    #[test]
    fn test_find_root_manifest() {
        let sample_manifest_path = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("06_workspace_folder_project_target")
            .join("folder2");
        let manifest_root = find_root_manifest(&sample_manifest_path);
        println!("{:?}", manifest_root);
        let parsed_result = read_manifest(&manifest_root.unwrap().to_path_buf()).unwrap();
        match parsed_result {
            crate::manifest::EitherManifest::Real(m) => match m {
                crate::manifest::Manifest::Project(_) => todo!(),
                crate::manifest::Manifest::Target(_) => todo!(),
            },
            crate::manifest::EitherManifest::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(_) => todo!(),
                crate::manifest::VirtualManifest::Folder(_) => todo!(),
            },
        }
        println!("{:?}", parsed_result);
    }
}
