use deno_core::Extension;

pub mod errors;
pub mod version;

pub fn init_ops() -> Vec<Extension> {
    vec![version::init_ops()]
}
