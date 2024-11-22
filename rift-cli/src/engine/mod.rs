pub mod ffi;

pub fn init() -> bool {
    unsafe { ffi::RiftEngineInit() }
}

pub fn load() {
    unsafe { ffi::RiftEngineLoad() }
}

pub fn shutdown() {
    unsafe { ffi::RiftEngineShutdown() }
}

pub fn get_user_commands() -> String {
    unsafe {
        let c_str = ffi::RiftEngineGetUserCommands();
        let c_str = std::ffi::CStr::from_ptr(c_str);
        c_str.to_str().unwrap().to_string()
    }
}

pub fn process_user_command(command: &str) {
    let c_str = std::ffi::CString::new(command).unwrap();
    unsafe { ffi::RiftEngineProcessUserCommand(c_str.as_ptr()) }
}
