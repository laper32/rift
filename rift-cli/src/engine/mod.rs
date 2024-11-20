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

pub fn get_tasks() -> String {
    unsafe {
        let c_str = ffi::RiftEngineGetTasks();
        let c_str = std::ffi::CStr::from_ptr(c_str);
        c_str.to_str().unwrap().to_string()
    }
}
