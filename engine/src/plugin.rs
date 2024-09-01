use crate::workspace::WorkspaceManager;

pub struct PluginManager {}

impl PluginManager {
    fn new() -> Self {
        Self {}
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<PluginManager> =
            once_cell::sync::Lazy::new(|| PluginManager::new());
        unsafe { &mut *INSTANCE }
    }

    fn load_plugins_from_workspace(&self) {
        let plugins = WorkspaceManager::instance().get_plugins();
        for plugin in plugins {
            // load plugin
        }
        // WorkspaceManager::instance()
    }
}
