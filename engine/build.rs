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
}
