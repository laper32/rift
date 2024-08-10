use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_dir);
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=engine.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=engine");
    }
}
