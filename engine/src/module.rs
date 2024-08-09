/**
 * 大概会做的事情，有什么东西，怎么协调。
 *
 * 目前有两个东西：ModuleManager, ModuleRegistry
 *
 * ModuleRegistry: 用于注册模块，专门给ffi的RegisterRiftModule用的。
 * 只有注册功能，没有其他功能。
 * 其他功能如加载Modules，初始化Modules，卸载Modules等，都在ModuleManager里。
 *
 * ModuleManager：真正管这些Modules的地方。
 * ModuleRegistry同样也需要ModuleManager初始化的时候才能注册上这些实例。
 * 其应当想办法转译ModuleRegistry的FFI信息到相对安全的环境下处理，aka, OnLoad, OnUnload, OnAllLoaded这些。
 *
 */
use crate::{dir::PathIdentity, ffi::RiftModule};
use std::{fs, path::PathBuf};

pub const MODULE_DIR_NAME: &str = "modules";
const MODULE_ENTRY_FN: &[u8] = b"ModuleMain";

#[static_init::dynamic(drop)]
static mut INSTANCE: ModuleManager = ModuleManager::new();

pub fn init() {
    println!("module::init()");
}

pub struct ModuleInstanceVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

pub struct ModuleInstanceDescriptor {
    name: String,
    version: ModuleInstanceVersion,
    description: String,
    url: String,
    instance_path: String,
}

pub struct ModuleInstance {
    descriptor: ModuleInstanceDescriptor,
    instance: RiftModule,
}

pub struct ModuleManager {
    modules: Vec<ModuleInstance>,
}

impl ModuleManager {
    fn new() -> Self {
        let installation_dir = PathIdentity::get_rift_path(PathIdentity::Installation);
        let user_profile_dir = PathIdentity::get_rift_path(PathIdentity::UserProfile);
        let mut possible_modules = Vec::new();

        ModuleManager::scan_possible_modules(&mut possible_modules, installation_dir);
        ModuleManager::scan_possible_modules(&mut possible_modules, user_profile_dir);
        // let mut pending_modules = Vec::new();

        possible_modules.iter().for_each(|x| {
            ModuleManager::load_module(x);
        });

        Self {
            modules: Vec::new(), // possible_modules,
        }
    }
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ModuleManager> =
            once_cell::sync::Lazy::new(|| ModuleManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn init(&mut self) {
        // ModuleRegistry::instance().modules.iter().for_each(|x| {
        //     self.modules.push(ModuleInstance {
        //         descriptor: ModuleInstanceDescriptor {
        //             name: "name".to_owned(),
        //             version: ModuleInstanceVersion {
        //                 major: 0,
        //                 minor: 0,
        //                 patch: 0,
        //             },
        //             description: "description".to_owned(),
        //             url: "url".to_owned(),
        //             instance_path: "instance_path".to_owned(),
        //         },
        //         instance: unsafe { **x },
        //     });
        // });
    }

    fn load_module(module_path: &PathBuf) {
        let dll = unsafe { libloading::Library::new(module_path) };
        match dll {
            Ok(dll) => {
                let module_main_fn =
                    unsafe { dll.get::<unsafe extern "C" fn() -> i32>(MODULE_ENTRY_FN) };
                match module_main_fn {
                    Ok(module_main_fn) => {
                        let ret = unsafe { module_main_fn() };
                        if ret != 0 {
                            println!("Error on loading"); // TODO: LogError => Failed loading.
                            return;
                        }

                        println!("ModuleMain: {}", ret);
                    }
                    Err(e) => {
                        println!("FindFunc Error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("LoadLibrary Error: {:?}", e);
            }
        }
    }

    fn scan_possible_modules(possible_modules: &mut Vec<PathBuf>, path: String) {
        let binding = PathBuf::from(path).join(MODULE_DIR_NAME);
        let path = binding.to_str().unwrap();
        let paths = fs::read_dir(path);
        match paths {
            Ok(paths) => {
                for path in paths {
                    match path {
                        Ok(path) => {
                            let path = path.path();
                            if Self::is_module(path.clone()) {
                                possible_modules.push(path);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {
                // ...
            }
        }
    }

    fn is_module(path: PathBuf) -> bool {
        #[cfg(target_os = "windows")]
        {
            let module_ext = "dll";
            match path.extension() {
                Some(ext) => ext == module_ext,
                None => false,
            }
        }
    }
}

// used for FFI
pub struct ModuleRegistry {
    modules: Vec<*const RiftModule>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ModuleRegistry> =
            once_cell::sync::Lazy::new(|| ModuleRegistry::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn register_module(&mut self, module: *const RiftModule) {
        self.modules.push(module);
    }
}
