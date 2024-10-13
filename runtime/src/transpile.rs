use std::path::Path;

use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::ModuleType;
use engine::shared::errors::RiftResult;

fn check_transpile_file(path: &Path) -> RiftResult<(ModuleType, bool)> {
    match MediaType::from_path(&path) {
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
        _ => anyhow::bail!("Unknown extension {:?}, path: {:?}", path.extension(), path),
    }
}

pub(crate) fn transpile_file(path: &Path) -> RiftResult<(ModuleType, String)> {
    match check_transpile_file(path) {
        Ok((module_type, should_transpile)) => {
            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: path.to_string_lossy().to_string().parse().unwrap(),
                    text: SourceTextInfo::from_string(code).text(),
                    media_type: MediaType::from_path(&path),
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
            Ok((module_type, code))
        }
        Err(e) => anyhow::bail!(e),
    }
}
