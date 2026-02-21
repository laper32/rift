use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TomlFolder {
    pub name: Option<String>,
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    #[serde(default)]
    pub others: HashMap<String, toml::Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TomlWorkspace {
    pub name: Option<String>,
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    #[serde(default)]
    pub others: HashMap<String, toml::Value>,
}
