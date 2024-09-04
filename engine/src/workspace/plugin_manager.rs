use std::collections::HashMap;

#[derive(Debug)]
pub struct ManifestPluginIdentifer {
    pub name: String,
    pub version: String,
}

/// 这里管的是插件本身的运行。。。
/// 为什么WorkspaceManager那边还要有一个Plugin？因为在运行插件之前，我们需要知道哪些插件需要加载，而知道插件需要加载的过程，就是Workspace处理的
pub struct PluginManager {
    plugins: HashMap<String, Vec<ManifestPluginIdentifer>>,
}

impl PluginManager {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<PluginManager> =
            once_cell::sync::Lazy::new(|| PluginManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn register_manifest_plugin(
        &mut self,
        pkg_name: String,
        identifier: ManifestPluginIdentifer,
    ) {
        self.plugins
            .entry(pkg_name)
            .or_insert(Vec::new())
            .push(identifier);
    }

    pub fn print(&self) {
        println!("Plugins: {:?}", self.plugins);
    }
}
