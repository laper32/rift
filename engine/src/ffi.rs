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
    pub name: *const c_char,
    pub version: RiftModuleVersion,
    pub description: *const c_char,
    pub author: *const c_char,
    pub url: *const c_char,
}

unsafe impl Send for RiftModuleDescriptor {}

#[repr(C)]
pub struct RiftModule {
    pub OnLoad: fn() -> bool,
    pub OnUnload: fn(),
    pub OnAllLoad: fn(),
    pub descriptor: *const RiftModuleDescriptor,
}

unsafe impl Send for RiftModule {}

#[no_mangle]
extern "C" fn RegisterRiftModule(module: *const RiftModule) {
    ModuleRegistry::instance().register_module(module);
}

#[no_mangle]
extern "C" fn DeclareRiftModuleDescriptor(
    name: *const c_char,
    version: RiftModuleVersion,
    description: *const c_char,
    author: *const c_char,
    url: *const c_char,
) -> *const RiftModuleDescriptor {
    println!("DeclareModuleDescriptor");
    let ret = Box::new(RiftModuleDescriptor {
        name,
        version,
        description,
        author,
        url,
    });
    Box::into_raw(ret)
}

#[no_mangle]
extern "C" fn __InitRiftEngine() -> bool {
    init()
}

#[no_mangle]
extern "C" fn __ShutdownRiftEngine() {
    shutdown()
}
