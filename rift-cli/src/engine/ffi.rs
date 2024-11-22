use std::ffi::c_char;

extern "C" {
    pub fn RiftEngineInit() -> bool;
    pub fn RiftEngineLoad();
    pub fn RiftEngineShutdown();
    pub fn RiftEngineGetUserCommands() -> *const c_char;
}
