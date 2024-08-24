// pub struct ModuleLoader {
//     pub sources:
// }

use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
    sync::Arc,
};

use anyhow::Context;

use crate::{forward::ForwardManager, util::errors::RiftResult};

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
        let contents =
            ForwardManager::instance().read_specifier_contents(rift_module_specifier.clone())?;
        let code = std::str::from_utf8(&contents)
            .context("failed to parse module contents as UTF-8 string")?;
        let parsed = deno_ast::parse_module(deno_ast::ParseParams {
            specifier: rift_module_specifier.clone().into(),
            text: code.into(),
            media_type: deno_ast::MediaType::TypeScript,
            capture_tokens: false,
            scope_analysis: false,
            maybe_syntax: None,
        })?;
        let transpiled = parsed.transpile(
            &deno_ast::TranspileOptions {
                imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Preserve,
                ..Default::default()
            },
            &deno_ast::EmitOptions {
                source_map: deno_ast::SourceMapOption::Separate,
                ..Default::default()
            },
        )?;

        if let Entry::Vacant(entry) = self
            .sources
            .borrow_mut()
            .entry(rift_module_specifier.clone())
        {
            let source_map = transpiled
                .clone()
                .into_source()
                .source_map
                .context("source map not generated")?;
            entry.insert(ModuleSource {
                source_contents: contents.clone(),
                source_map,
            });
        }

        Ok(deno_core::ModuleSource::new(
            deno_core::ModuleType::JavaScript,
            deno_core::ModuleSourceCode::Bytes(
                transpiled.into_source().source.into_boxed_slice().into(),
            ),
            module_specifier,
            None,
        ))
    }
}

impl deno_core::ModuleLoader for RiftModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        todo!()
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        is_dyn_import: bool,
        requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        todo!()
    }
}

struct ModuleSource {
    pub source_contents: Arc<Vec<u8>>,
    pub source_map: Vec<u8>,
}
