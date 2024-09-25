use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

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
    pub alias: Option<TomlAlias>,
    pub task: Option<TomlTask>,
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
    // 如果是模板生成的话就是src/main.ts，当然用户可以自己编辑
    pub entry: String,
}

/// 给脚本用的<br/>
/// 因为脚本是TS写的，所以我们需要一个TS的类型来描述这个结构<br/>
/// 然后这里就有一个问题：如果这个插件/依赖的来源并不是从包管理器那来的，<br/>
/// 比如说从特定的路径来，又或者是从git那边来<br/>
/// 这时候这么写会被直接干碎
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TomlPluginManifestDeclarator {
    pub name: String,
    #[serde(flatten)]
    pub data: HashMap<
        String,            // 依赖的信息
        serde_json::Value, // 对应的值
    >,
}

/// 给脚本用的<br/>
/// 因为脚本是TS写的，所以我们需要一个TS的类型来描述这个结构<br/>
/// 然后这里就有一个问题：如果这个插件/依赖的来源并不是从包管理器那来的，<br/>
/// 比如说从特定的路径来，又或者是从git那边来<br/>
/// 这时候这么写会被直接干碎
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TomlDependencyManifestDeclarator {
    pub name: String,
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
    // pub version: Option<String>, // 总得考虑这玩意是个本地包吧？
}

pub type TomlAlias = HashMap<String, String>;
pub type TomlTask = HashMap<String, TomlTaskInstance>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlTaskInstance {
    pub about: Option<String>,
    pub is_command: Option<bool>,
    pub args: Option<Vec<TomlTaskFlag>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlTaskFlag {
    pub name: String,
    pub short: Option<String>,
    #[serde(rename = "help")]
    pub description: Option<String>,
    pub default: Option<toml::Value>,
    pub conflict_with: Option<Vec<String>>,
    #[serde(rename = "help_heading")]
    pub heading: Option<String>,
}

pub fn load_manifest(path: &Path) -> RiftResult<TomlManifest> {
    let raw_content = std::fs::read_to_string(path)?;
    let content = toml::from_str::<TomlManifest>(raw_content.as_str());
    Ok(content?)
}

#[cfg(test)]
mod test {
    // use crate::schema::load_manifest;
    use std::{collections::HashMap, path::PathBuf};

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

    #[test]
    fn test_capture_not_predifined_fields() {
        #[derive(std::fmt::Debug, serde::Deserialize)]
        #[allow(dead_code)]
        struct Example {
            a: String,
            b: String,
            c: i32,
            #[serde(flatten)]
            others: HashMap<String, serde_json::Value>,
        }
        let data = r#"{
        "a": "hello",
        "b": "world",
        "c": 1,
        "d": "ddd",
        "git": "git://github.com/xxx/xxx.git",
        "nested": {
            "key": "Value"
            }
        }"#;
        let descrialzed = serde_json::from_str::<Example>(data);
        println!("{:?}", descrialzed);
    }
}
