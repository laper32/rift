pub enum RiftManifest {
    Plugin(PluginManifest),
}
pub struct PluginManifest {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    pub others: std::collections::HashMap<String, toml::Value>,
}
