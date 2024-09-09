use std::{env, path::PathBuf};

use deno_core::extension;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runtime_dir = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("runtime");
    let out_dir: PathBuf = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNTIME_SNAPSHOT.bin");
    let runtime_file_path = runtime_dir.join("dist").join("index.js");

    println!("cargo:rerun-if-changed={}", runtime_file_path.display());

    extension!(runtime, js = ["../runtime/dist/index.js"],);

    let snapshot = deno_core::snapshot::create_snapshot(
        deno_core::snapshot::CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            startup_snapshot: None,
            skip_op_registration: false,
            extensions: vec![runtime::init_ops_and_esm()],
            with_runtime_cb: None,
            extension_transpiler: None,
        },
        None,
    )
    .unwrap();

    std::fs::write(snapshot_path, snapshot.output).unwrap();
}
