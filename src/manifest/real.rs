pub enum Manifest {
    Target(TargetManifest),
    Project(ProjectManifest),
}

pub struct ProjectManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub others: std::collections::HashMap<String, toml::Value>,
}

pub struct TargetManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub others: std::collections::HashMap<String, toml::Value>,
}
