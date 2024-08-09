#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_char;
use std::ffi::c_float;
use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

use crate::init;
use crate::module::ModuleRegistry;
use crate::shutdown;

#[repr(C)]
pub struct RiftModule {
    pub OnLoad: fn() -> bool,
    pub OnUnload: fn(),
    pub OnAllLoad: fn(),
}

unsafe impl Send for RiftModule {}

#[no_mangle]
extern "C" fn RegisterRiftModule(module: RiftModule) {
    ModuleRegistry::instance().register_module(module);
}

#[no_mangle]
extern "C" fn __InitRiftEngine() -> bool {
    init()
}

#[no_mangle]
extern "C" fn __ShutdownRiftEngine() {
    shutdown()
}
