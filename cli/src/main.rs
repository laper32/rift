mod ffi;

use clap::Command;
use ffi::engine;
fn main() {
    let _matches = Command::new("Rift")
        .version("1.0") // TODO: Replace with actual project version
        .about("Rift build system")
        .get_matches();
    if !engine::init_engine() {
        println!("Failed to initialize engine");
        return;
    }

    engine::shutdown_engine()
}
