use crate::errors::{ManifestError, RiftResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// This type is used to deserialize `Rift.toml` files.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlManifest {
    pub workspace: Option<TomlWorkspace>,
    pub folder: Option<TomlFolder>,
    pub project: Option<TomlProject>,
    pub target: Option<TomlTarget>,
    pub plugin: Option<TomlPlugin>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TomlWorkspace {
    /// 没啥说的
    pub members: Option<Vec<String>>,

    /// 没啥说的
    pub exclude: Option<Vec<String>>,

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
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
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
    pub metadata: Option<String>
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
    let raw_content = std::fs::read_to_string(path).map_err(|err| ManifestError::new(err, path.into()))?;
    let content = toml::from_str::<TomlManifest>(raw_content.as_str());

    Ok(content?)
    // let content = toml::from_str(std::fs::read_to_string(path.to_path_buf()).unwrap() )
}

#[test]
fn test_load_manifest() {
    use std::env;

    let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let identifier_path = PathBuf::from(manifest_path) // rift/engine
        .parent().unwrap() // rift
        .join(".dist")
        .join("sample_project")
        .join("Rift.toml");
    println!("Identifier path: {identifier_path:?}");
    let manifest = load_manifest(&identifier_path);
    println!("{:?}", manifest);

}