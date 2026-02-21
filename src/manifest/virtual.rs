use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

pub enum VirtualManifest {
    Folder(FolderManifest),
    Workspace(WorkspaceManifest),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderManifest {
    pub name: String,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    #[serde(default)]
    pub others: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub name: String,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    #[serde(default)]
    pub others: HashMap<String, Value>,
}
