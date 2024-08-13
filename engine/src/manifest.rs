use std::path::{Path, PathBuf};

pub const MANIFEST_IDENTIFIER: &str = "Rift.toml";

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
    // 多个target才会用，如果只有一个的话建议不用，虽然我们不反对。
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
    Plugin(PluginManifest),
}

/// 这两个严格来说只用来组织项目结构
pub enum Connector {
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

#[cfg(test)]
mod test {
    use std::{env, io};
    use std::ffi::OsString;
    use std::fs::read_dir;
    use std::io::ErrorKind;
    use std::path::PathBuf;
    use crate::manifest::find_root_manifest;

    fn get_project_root() -> io::Result<PathBuf> {
        let path = env::current_dir()?;
        let mut path_ancestors = path.as_path().ancestors();

        while let Some(p) = path_ancestors.next() {
            let has_cargo =
                read_dir(p)?
                    .into_iter()
                    .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
            if has_cargo {
                return Ok(PathBuf::from(p))
            }
        }
        Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))

    }
    #[test]
    fn test_find_root_manifest() {
        let sample_manifest_path = get_project_root().unwrap().join("sample").join("06_workspace_folder_project_target").join("folder2");
        let manifest_root = find_root_manifest(&sample_manifest_path);
        println!("{:?}", manifest_root);
        
    }
}