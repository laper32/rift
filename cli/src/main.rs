use clap::Command;
fn main() {
    let _matches = Command::new("Rift")
        .version("1.0") // TODO: Replace with actual project version
        .about("Rift build system")
        .get_matches();

    engine::init();

    engine::shutdown();
}
