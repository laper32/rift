use deno_core::{extension, op2, Extension};
use once_cell::sync::Lazy;

const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
const GIT_COMMIT_HASH_SHORT: &str = env!("GIT_COMMIT_HASH_SHORT");

pub const RIFT_VERSION: Lazy<RiftVersion> = Lazy::new(|| {
    // ...
    RiftVersion {
        version: concat!(
            env!("CARGO_PKG_VERSION"),
            ".",
            env!("GIT_COMMIT_HASH_SHORT")
        ),
        version_full: concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_COMMIT_HASH")),
        git_hash: GIT_COMMIT_HASH,
        git_hash_short: GIT_COMMIT_HASH_SHORT,
    }
});

pub struct RiftVersion {
    pub version: &'static str,
    pub version_full: &'static str,
    pub git_hash: &'static str,
    pub git_hash_short: &'static str,
}

#[op2]
#[string]
fn op_rift_version() -> String {
    RIFT_VERSION.version.to_string()
}

#[op2]
#[string]
fn op_rift_version_full() -> String {
    RIFT_VERSION.version_full.to_string()
}

#[op2]
#[string]
fn op_rift_git_hash() -> String {
    RIFT_VERSION.git_hash.to_string()
}

#[op2]
#[string]
fn op_rift_git_hash_short() -> String {
    RIFT_VERSION.git_hash_short.to_string()
}

extension!(
    version,
    ops = [
        op_rift_version,
        op_rift_version_full,
        op_rift_git_hash,
        op_rift_git_hash_short
    ]
);

pub fn init_ops() -> Extension {
    version::init_ops_and_esm()
}
