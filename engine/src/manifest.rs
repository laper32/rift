use std::path::{Path, PathBuf};

use crate::schema;

pub const MANIFEST_IDENTIFIER: &str = "Rift.toml";

pub enum EitherManifest {
    Real(Manifest),
    Virtual(VirtualManifest),
}

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

pub struct FolderManifest {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

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
pub enum Manifest {
    Project(ProjectManifest),
    Target(TargetManifest),
}

/// 这两个严格来说只用来组织项目结构
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

pub fn read_manifest(path: &Path) -> Option<EitherManifest> {
    let mut manifest = || {
        let deserialized_toml = schema::load_manifest(&path.to_path_buf());
        match deserialized_toml {
            std::result::Result::Ok(manifest) => {
                if manifest.workspace.is_some() {
                    let workspace_manifest = manifest.workspace.unwrap();
                    return EitherManifest::Virtual(VirtualManifest::Workspace(
                        WorkspaceManifest {
                            members: workspace_manifest.members,
                            exclude: Some(workspace_manifest.exclude),
                            metadata: workspace_manifest.metadata,
                            plugins: workspace_manifest.plugins,
                            dependencies: workspace_manifest.dependencies,
                        },
                    ));
                } else if manifest.folder.is_some() {
                    let folder_manifest = manifest.folder.unwrap();
                    return EitherManifest::Virtual(VirtualManifest::Folder(FolderManifest {
                        members: Some(folder_manifest.members),
                        exclude: Some(folder_manifest.exclude),
                    }));
                } else if manifest.project.is_some() {
                    todo!()
                } else if manifest.target.is_some() {
                    todo!()
                } else {
                    panic!("Invalid manifest");
                }
            }
            Err(e) => {
                todo!("error: {:?}", e);
            }
        }
        // Some(match deserialized_toml {
        //     Ok(manifest) => {
        //         if manifest.workspace.is_some() {
        //             let workspace_manifest = manifest.workspace.unwrap();
        //             EitherManifest::Virtual(VirtualManifest::Workspace(WorkspaceManifest {
        //                 members: workspace_manifest.members,
        //                 exclude: Some(workspace_manifest.exclude),
        //                 metadata: workspace_manifest.metadata,
        //                 plugins: workspace_manifest.plugins,
        //                 dependencies: workspace_manifest.dependencies,
        //             }))
        //         } else {
        //             return None;
        //         }
        //     }
        //     Err(e) => {
        //         return None;
        //     }
        // })
    };
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{manifest::find_root_manifest, util::get_cargo_project_root};

    #[test]
    fn test_find_root_manifest() {
        let sample_manifest_path = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("06_workspace_folder_project_target")
            .join("folder2");
        let manifest_root = find_root_manifest(&sample_manifest_path);
        println!("{:?}", manifest_root);
    }
}
