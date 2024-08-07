use clap::Command;
use std::ffi::{c_char, c_int, c_void};

#[allow(dead_code)]
extern "C" {
    fn CreateInterface(name: *const c_char, return_code: *mut c_int) -> *mut c_void;
}

fn main() {
    let matches = Command::new("Rift")
        .version("1.0") // TODO: Replace with actual project version
        .about("Rift build system")
        .get_matches();
    unsafe {
        let mut ret_code = 0;
        CreateInterface("Naming".as_ptr() as *const c_char, &mut ret_code);
    }
    // let matches = Command::new("MyApp")
    //     .version("1.0")
    //     .about("Does awesome things")
    //     .arg(arg!(--two <VALUE>).required(true))
    //     .arg(arg!(--one <VALUE>).required(true))
    //     .get_matches();

    // println!(
    //     "two: {:?}",
    //     matches.get_one::<String>("two").expect("required")
    // );
    // println!(
    //     "one: {:?}",
    //     matches.get_one::<String>("one").expect("required")
    // );
}
