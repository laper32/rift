use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlProject {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub others: std::collections::HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlTarget {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub others: std::collections::HashMap<String, toml::Value>,
}
