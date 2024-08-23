pub mod evaluate;
pub mod loader;
mod script;
pub mod specifier;

#[allow(dead_code)]
use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::anyhow::Error;
use deno_core::error::AnyError;
use deno_core::{
    extension, op2, FastString, ModuleLoadResponse, ModuleSourceCode, ModuleSpecifier, ModuleType,
    ResolutionKind,
};
use std::env;
use std::rc::Rc;

// RA会在需要展开的时候变成弱智
// 所以需要非常小心的用
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../runtime"]
#[include = "dist/**/*.js"]
pub struct RuntimeFiles;

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
#[string]
async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[op2(async)]
async fn op_set_timeout(delay: f64) -> Result<(), AnyError> {
    tokio::time::sleep(std::time::Duration::from_millis(delay as u64)).await;
    Ok(())
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

// TODO: 这个是准备remake的，因为我们现在只能加载有限个文件，还没办法加载以包为单位的文件。
// 换句话说没法解决复杂需求。
struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&reqwest::Url>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let module_load = Box::pin(async move {
            let path = module_specifier.to_file_path().unwrap();

            let media_type = MediaType::from_path(&path);

            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (deno_core::ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::Json => (deno_core::ModuleType::Json, false),
                _ => panic!("Unknown extension {:?}", path.extension()),
            };

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
        });
        ModuleLoadResponse::Async(module_load)
    }
}
static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));

extension! {
    runjs,
    ops = [
        op_read_file,
        op_write_file,
        op_remove_file,
        op_fetch,
        op_set_timeout,
    ]
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        startup_snapshot: Some(RUNTIME_SNAPSHOT),
        extensions: vec![runjs::init_ops()],
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    // 只有等全部解析加载完成我们才能得知要运行哪些文件哦
    if let Err(error) = runtime.block_on(run_js(todo!())) {
        eprintln!("error: {error}");
    }
}

pub fn shutdown() {}
