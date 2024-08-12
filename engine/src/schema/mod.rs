use crate::errors::{RiftResult, SimpleError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde_with::serde_as]
pub struct TomlWorkspace {
    /// 没啥说的
    /// 强制加，不能没有这个field，否则报错。
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub members: Vec<String>,

    /// 没啥说的
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub exclude: Vec<String>,

    /// 指向的是文件路径
    pub metadata: Option<String>,

    /// 指向的是文件路径
    pub plugins: Option<String>,

    /// 指向的是文件路径
    pub dependencies: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::schema::TomlWorkspace;

    #[test]
    fn __test_maybe_null_workspace() {
        let raw = r#"
        [workspace]
        "#;
        let content = toml::from_str::<TomlWorkspace>(raw).unwrap();
    }
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlFolder {
    pub members: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlProject {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
    // 多个target才会用，如果只有一个的话建议不用，虽然我们不反对。
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub members: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlTarget {
    pub name: String,
    #[serde(rename = "type")]
    pub build_type: String,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlPlugin {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub dependencies: Option<String>,
}

// pub fn load_manifest(path: &Path) -> RiftResult<TomlManifest> {
//     let raw_content =
//         std::fs::read_to_string(path).map_err(|err| ManifestError::new(err, path.into()))?;
//     let content = toml::from_str::<TomlManifest>(raw_content.as_str());
// 
//     Ok(content?)
//     // let content = toml::from_str(std::fs::read_to_string(path.to_path_buf()).unwrap() )
// }

pub fn load_manifest<T>(path: &Path) -> RiftResult<T> {
    let raw_content =
        std::fs::read_to_string(path).map_err(|err| SimpleError::new("..."))?;
    toml::from_str::<T>(raw_content.as_str())?
}


#[test]
fn test_load_manifest() {
    use std::env;

    let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let identifier_path = PathBuf::from(manifest_path) // rift/engine
        .parent()
        .unwrap() // rift
        .join(".dist")
        .join("sample_project")
        .join("Rift.toml");
    println!("Identifier path: {identifier_path:?}");
    let manifest = load_manifest(&identifier_path);
    println!("{:?}", manifest);
}
