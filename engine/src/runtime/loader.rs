// pub struct ModuleLoader {
//     pub sources:
// }

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::util::errors::RiftResult;

use super::specifier::RiftModuleSpecifier;

pub struct RiftModuleLoader {
    // sources: Vec<ModuleSource>,
    pub sources: Rc<RefCell<HashMap<RiftModuleSpecifier, ModuleSource>>>,
}

impl RiftModuleLoader {
    pub fn new() -> Self {
        Self {
            sources: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn load_module_source(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
    ) -> RiftResult<deno_core::ModuleSource> {
        let rift_module_specifier: RiftModuleSpecifier = module_specifier.try_into()?;
        /*
        let contents = self
            .bridge
            .read_specifier_contents(brioche_module_specifier.clone())?;
         */
        todo!()
    }
}

struct ModuleSource {
    pub source_contents: Arc<Vec<u8>>,
    pub source_map: Arc<u8>,
}
