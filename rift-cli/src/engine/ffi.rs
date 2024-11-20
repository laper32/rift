use std::ffi::c_char;

extern "C" {
    pub fn RiftEngineInit() -> bool;
    pub fn RiftEngineLoad();
    pub fn RiftEngineShutdown();
    pub fn RiftEngineGetTasks() -> *const c_char;
}
