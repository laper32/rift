use deno_core::{extension, Extension};

pub mod shared;

pub fn init_ops() -> Vec<Extension> {
    let mut ret = Vec::new();
    let shared_ops = shared::init_ops();
    for op in shared_ops {
        ret.push(op);
    }
    ret
}

pub static ENGINE_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ENGINE_SNAPSHOT.bin"));
