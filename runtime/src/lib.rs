use shared::RuntimeFiles;
use vm::ScriptRuntime;

mod loader;
mod shared;
mod vm;

pub fn init() {
    let rt_index = RuntimeFiles::get("dist/index.js").unwrap();
    let rt_idx_file: deno_core::ModuleSpecifier = "dist/index.js".parse().unwrap();
    ScriptRuntime::instance()
        .js_runtime()
        .load_main_es_module(&rt_idx_file);
    println!("{}", std::str::from_utf8(rt_index.data.as_ref()).unwrap());
}

pub fn shutdown() {}
