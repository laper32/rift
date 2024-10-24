fn main() {
    let lib_dir =
        std::env::var("RIFT_ENGINE_OUT_DIR").unwrap_or("../rift-engine/build/Release".to_string());
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo:rustc-link-lib=rift.engine");
}
