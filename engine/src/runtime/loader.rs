use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::{
    FastString, ModuleLoadResponse, ModuleResolutionError, ModuleSourceCode, ModuleSpecifier,
};
use url::{ParseError, Url};

/// From deno_core's self module resolving rule, but made some modifications.
pub fn resolve_import(
    specifier: &str,
    base: &str,
) -> Result<ModuleSpecifier, ModuleResolutionError> {
    let url = match Url::parse(specifier) {
        // 1. Apply the URL parser to specifier.
        //    If the result is not failure, return he result.
        Ok(url) => url,

        // 2. If specifier does not start with the character U+002F SOLIDUS (/),
        //    the two-character sequence U+002E FULL STOP, U+002F SOLIDUS (./),
        //    or the three-character sequence U+002E FULL STOP, U+002E FULL STOP,
        //    U+002F SOLIDUS (../), return failure.
        Err(ParseError::RelativeUrlWithoutBase)
            if !(specifier.starts_with('/')
                || specifier.starts_with("./")
                || specifier.starts_with("../")) =>
        {
            let maybe_referrer = if base.is_empty() {
                None
            } else {
                Some(base.to_string())
            };
            return Err(ModuleResolutionError::ImportPrefixMissing(
                specifier.to_string(),
                maybe_referrer,
            ));
        }

        // 3. Return the result of applying the URL parser to specifier with base
        //    URL as the base URL.
        Err(ParseError::RelativeUrlWithoutBase) => {
            let base = Url::parse(base).map_err(ModuleResolutionError::InvalidBaseUrl)?;
            base.join(specifier)
                .map_err(ModuleResolutionError::InvalidUrl)?
        }

        // If parsing the specifier as a URL failed for a different reason than
        // it being relative, always return the original error. We don't want to
        // return `ImportPrefixMissing` or `InvalidBaseUrl` if the real
        // problem lies somewhere else.
        Err(err) => return Err(ModuleResolutionError::InvalidUrl(err)),
    };

    Ok(url)
}

pub struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        resolve_import(specifier, referrer).map_err(|e| e.into())
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
            println!("load: {:?}", module_specifier);
            if module_specifier.scheme() == "rift" {
                // rift: => 至少现在，我们将会以Workspace的package为单位搜包
            }

            let path = module_specifier.to_file_path();
            if path.is_err() {
                return Err(
                    ModuleResolutionError::InvalidUrl(ParseError::RelativeUrlWithoutBase).into(),
                );
            }
            let path = path.unwrap();

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
