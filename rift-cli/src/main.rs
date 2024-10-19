mod engine {
    mod ffi {

        extern "C" {
            pub fn RiftEngineInit() -> bool;
            pub fn RiftEngineShutdown();
        }
    }
    pub fn init() -> bool {
        unsafe { ffi::RiftEngineInit() }
    }

    pub fn shutdown() {
        unsafe { ffi::RiftEngineShutdown() }
    }
}

fn main() {
    engine::init();

    engine::shutdown();
}
