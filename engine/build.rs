use std::{env, path::PathBuf};

use deno_core::extension;
use npm_rs::{NodeEnv, NpmEnv};

fn compile_runtime() {
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("runtime"),
        )
        // .with_env("FOO", "bar")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}

fn main() {
    compile_runtime();
    extension!(runjs, js = ["../runtime/dist/index.js",]);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

    let snapshot = deno_core::snapshot::create_snapshot(
        deno_core::snapshot::CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            startup_snapshot: None,
            skip_op_registration: false,
            extensions: vec![runjs::init_ops_and_esm()],
            with_runtime_cb: None,
            extension_transpiler: None,
        },
        None,
    )
    .unwrap();

    std::fs::write(snapshot_path, snapshot.output).unwrap();
}
