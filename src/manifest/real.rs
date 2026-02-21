use std::collections::HashMap;
use toml::Value;

pub enum Manifest {
    Target(TargetManifest),
    Project(ProjectManifest),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectManifest {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    /// Target and members/exclude are mutually exclusive:
    /// - If `target` is Some, then `members` and `exclude` must be None
    /// - If `members` or `exclude` is Some, then `target` must be None
    pub target: Option<TargetManifest>,
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub others: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TargetManifest {
    pub name: String,
    pub target_type: String,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub tasks: Option<String>,
    pub others: HashMap<String, Value>,
}
