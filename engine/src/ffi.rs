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
use std::ptr;

/// The Scene Graph.
#[repr(C)]
pub struct SceneGraph {
    id: c_int,
}

// #[no_mangle]
// pub extern "C" fn scene_graph_new(id: c_int) -> *mut SceneGraph {
//     println!("SCENEGRAPH: Add SceneGraph {:?}", id);
//     let scene_graph = Box::new(SceneGraph { id });
//     Box::into_raw(scene_graph)
// }

// #[no_mangle]
// pub extern "C" fn scene_graph_delete(scene_graph_ptr: *mut SceneGraph) {
//     println!("SCENEGRAPH: Remove SceneGraph {:?}", scene_graph_ptr);
//     if scene_graph_ptr.is_null() {
//         return;
//     }
//     unsafe {
//         // Brings scene_graph_ptr into scope and then forces Rust to deallocate it
//         // when it falls out of the function scope.
//         Box::from_raw(scene_graph_ptr);
//     }
// }

// /// Module descriptor
// #[repr(C)]
// pub struct RiftModuleDescriptor {
//     name: *const c_char,
// }

pub struct RiftModuleDescriptor {}

#[repr(C)]
pub struct RiftModule {
    OnLoad: fn() -> bool,
    OnUnload: fn(),
    OnAllLoad: fn(),
    descriptor: *const RiftModuleDescriptor,
}

#[no_mangle]
extern "C" fn __dummy_to_let_this_idiot_generated_DO_NOT_USE_IT(_: RiftModule) {}
