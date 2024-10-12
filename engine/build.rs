use std::{env, path::PathBuf};

use deno_core::extension;

fn git_commit_hash() -> String {
    if let Ok(output) = std::process::Command::new("git")
        .arg("rev-list")
        .arg("-1")
        .arg("HEAD")
        .output()
    {
        if output.status.success() {
            std::str::from_utf8(&output.stdout[..40])
                .unwrap()
                .to_string()
        } else {
            // When not in git repository
            // (e.g. when the user install by `cargo install deno`)
            "LOCAL".to_string()
        }
    } else {
        // When there is no git command for some reason
        "LOCAL".to_string()
    }
}

fn main() {
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_commit_hash());
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");
    println!(
        "cargo:rustc-env=GIT_COMMIT_HASH_SHORT={}",
        &git_commit_hash()[..7]
    );

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runtime_dir = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("runtime");
    let out_dir: PathBuf = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("ENGINE_SNAPSHOT.bin");
    let runtime_file_path = runtime_dir.join("dist").join("index.js");

    println!("cargo:rerun-if-changed={}", runtime_file_path.display());

    extension!(runtime, js = ["./js/dist/index.js"],);

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
