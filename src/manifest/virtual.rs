use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

pub enum VirtualManifest {
    Folder(FolderManifest),
    Workspace(WorkspaceManifest),
}

// 1. 定义具体的结构体（保持清晰的字段结构）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderManifest {
    pub name: String,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
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
    #[serde(default)]
    pub others: HashMap<String, Value>,
}
