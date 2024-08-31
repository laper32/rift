use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::util::errors::RiftResult;

/// 我们定义项目是基于section的
/// 所以我们需要一个TomlManifest来明确Rift.toml里可能会有哪些section
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde_with::serde_as]
pub struct TomlManifest {
    pub workspace: Option<TomlWorkspace>,
    pub folder: Option<TomlFolder>,
    pub plugin: Option<TomlPlugin>,
    pub project: Option<TomlProject>,
    pub target: Option<TomlTarget>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde_with::serde_as]
pub struct TomlWorkspace {
    pub name: Option<String>,
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

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlFolder {
    pub name: Option<String>,
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

    // Members/Exclude不能和Target同时存在。
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
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

pub fn load_manifest(path: &Path) -> RiftResult<TomlManifest> {
    let raw_content = std::fs::read_to_string(path)?;
    let content = toml::from_str::<TomlManifest>(raw_content.as_str());
    Ok(content?)
}

#[cfg(test)]
mod test {
    // use crate::schema::load_manifest;
    use std::path::PathBuf;

    use super::TomlManifest;

    #[test]
    fn __test_maybe_null_workspace() {
        let raw = r#"
        [workspace]
        members = ["engine", "cli"]
        "#;
        let content = toml::from_str::<TomlManifest>(raw).unwrap();
        println!("{:?}", content);
        // let content = toml::from_str::<TomlWorkspace>(raw).unwrap();
        // println!("{:?}", content);
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

        // let manifest = load_manifest(&identifier_path);
        // println!("{:?}", manifest);
    }
}
