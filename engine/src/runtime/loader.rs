use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::{FastString, ModuleLoadResponse, ModuleSourceCode};
use url::Url;

pub struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        let url = Url::parse(specifier);
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let module_load = Box::pin(async move {
            let path = module_specifier.to_file_path().unwrap();

            let media_type = MediaType::from_path(&path);

            let transpile_result = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    Ok((deno_core::ModuleType::JavaScript, false))
                }
                MediaType::Jsx => Ok((deno_core::ModuleType::JavaScript, true)),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => Ok((deno_core::ModuleType::JavaScript, true)),
                MediaType::Json => Ok((deno_core::ModuleType::Json, false)),
                _ => Err(format!(
                    "Unknown extension {:?}, path: {:?}",
                    path.extension(),
                    path
                )),
            };
            match transpile_result {
                Ok((module_type, should_transpile)) => {
                    let code = std::fs::read_to_string(&path)?;
                    let code = if should_transpile {
                        let parsed = deno_ast::parse_module(ParseParams {
                            specifier: module_specifier.clone(),
                            text: SourceTextInfo::from_string(code).text(),
                            media_type,
                            capture_tokens: false,
                            scope_analysis: false,
                            maybe_syntax: None,
                        })?;
                        parsed
                            .transpile(&Default::default(), &Default::default())?
                            .into_source()
                            .into_string()
                            .unwrap()
                            .text
                    } else {
                        code
                    };
                    let module = deno_core::ModuleSource::new(
                        module_type,
                        ModuleSourceCode::String(FastString::from(code)),
                        &module_specifier,
                        None,
                    );
                    Ok(module)
                }
                Err(e) => {
                    anyhow::bail!(e)
                }
            }
        });
        ModuleLoadResponse::Async(module_load)
    }
}
