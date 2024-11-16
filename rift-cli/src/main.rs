// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#[allow(dead_code)]

mod engine {
    mod ffi {
        use std::ffi::c_char;

        extern "C" {
            pub fn RiftEngineInit() -> bool;
            pub fn RiftEngineShutdown();
            pub fn RiftEngineGetTasks() -> *const c_char;
        }
    }
    pub fn init() -> bool {
        unsafe { ffi::RiftEngineInit() }
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
}

fn main() {
    engine::init();
    println!("{}", engine::get_tasks());

    engine::shutdown();
}
