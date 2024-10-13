use anyhow::Context;
use deno_core::ModuleLoadResponse;
use engine::shared::errors::RiftResult;

use crate::{
    forward::ForwardManager,
    specifier::{RiftImportSpecifier, RiftModuleSpecifier},
};

pub(crate) struct RuntimeModuleLoader;

impl RuntimeModuleLoader {
    fn load_module_source(
        &self,
        specifier: &deno_core::ModuleSpecifier,
    ) -> RiftResult<deno_core::ModuleSource> {
        tracing::info!(%specifier, "loading module source");
        let specifier_into: RiftModuleSpecifier = specifier.try_into()?;
        let contents =
            ForwardManager::instance().read_specifier_contents(specifier_into.clone())?;
        let code = std::str::from_utf8(&contents)
            .context("failed to parse module contents as UTF-8 string")?;
        let parsed = deno_ast::parse_module(deno_ast::ParseParams {
            specifier: specifier_into.clone().into(),
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
        Ok(deno_core::ModuleSource::new(
            deno_core::ModuleType::JavaScript,
            deno_core::ModuleSourceCode::Bytes(
                transpiled.into_source().source.into_boxed_slice().into(),
            ),
            specifier,
            None,
        ))
    }
}

impl deno_core::ModuleLoader for RuntimeModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        if let deno_core::ResolutionKind::MainModule = kind {
            let resolved = specifier.parse()?;
            tracing::info!(%specifier, %referrer, %resolved, "resolved main module");
            return Ok(resolved);
        }
        let referrer: RiftModuleSpecifier = referrer.parse()?;
        let specifier: RiftImportSpecifier = specifier.parse()?;
        let resolved =
            ForwardManager::instance().resolve_specifier(specifier.clone(), referrer.clone())?;
        tracing::info!(%specifier, %referrer, %resolved, "resolved module");
        let resolved: deno_core::ModuleSpecifier = resolved.into();
        Ok(resolved)
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        ModuleLoadResponse::Sync(self.load_module_source(module_specifier))
    }
}
