use vm::ScriptRuntime;

mod forward;
mod loader;
mod shared;
mod specifier;
mod vm;

pub fn init() {
    let entry_path: deno_core::ModuleSpecifier = "rift:dist/index.js"
        .parse()
        .expect("failed to parse specifier");
    let result = ScriptRuntime::instance().evaluate(|| async {
        ScriptRuntime::instance().load_bootstarp().await?;
        Ok(())
    });
    println!("{:?}", result);
    ScriptRuntime::instance()
        .bootstrap_init()
        .expect("failed to bootstrap init");

    println!("entry_path: {:?}", entry_path);
}

pub fn shutdown() {
    // ScriptRuntime::instance()
    //     .bootstrap_shutdown()
    //     .expect("failed to bootstrap shutdown");
}
