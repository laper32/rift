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
pub struct RiftModuleVersion {
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int,
}

unsafe impl Send for RiftModuleVersion {}

#[repr(C)]
pub struct RiftModuleDescriptor {
    pub name: [c_char; 256],
    pub version: RiftModuleVersion,
    pub description: [c_char; 4096],
    pub author: [c_char; 256],
    pub url: [c_char; 256],
}

unsafe impl Send for RiftModuleDescriptor {}

#[repr(C)]
pub struct RiftModule {
    pub OnLoad: fn() -> bool,
    pub OnUnload: fn(),
    pub OnAllLoad: fn(),
    pub descriptor: RiftModuleDescriptor,
}

unsafe impl Send for RiftModule {}

#[no_mangle]
extern "C" fn RegisterRiftModule(module: *const RiftModule) {
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
