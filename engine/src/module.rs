use crate::ffi::RiftModule;

pub const MODULE_DIR_NAME: &str = "modules";

pub struct ModuleManager {
    modules: Vec<RiftModule>,
}

impl ModuleManager {
    fn init() -> bool {
        true
    }
}
