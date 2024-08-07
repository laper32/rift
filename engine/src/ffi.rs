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

#[repr(C)]
pub struct RiftModuleVersion {
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int,
}

#[repr(C)]
pub struct RiftModuleDescriptor {
    name: *const c_char,
    version: RiftModuleVersion,
    description: *const c_char,
    author: *const c_char,
    url: *const c_char,
}

#[repr(C)]
pub struct RiftModule {
    OnLoad: fn() -> bool,
    OnUnload: fn(),
    OnAllLoad: fn(),
    descriptor: *const RiftModuleDescriptor,
}
#[repr(C)]
pub struct IEngine {
    GetSearchPath: fn() -> *const c_char,
}

#[no_mangle]
extern "C" fn RegisterRiftModule(module: *const RiftModule) {
    println!("RegisterModule");
}
#[no_mangle]
extern "C" fn GetEngine() -> *const IEngine {
    ptr::null()
}
///
/// Create an interface to expose library.
/// Input: name: Name of the interface, which is the unique identifier.
/// Input: return_code: Return code of the interface.
/// Output: void*: Pointer to the interface (In this case it is IEngine).
#[no_mangle]
extern "C" fn CreateInterface(name: *const c_char, return_code: *mut c_int) -> *mut c_void {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().unwrap();
    println!("CreateInterface: {}", name);
    ptr::null_mut()
}
// void* CreateInterface(const char* name, int* return_code);
