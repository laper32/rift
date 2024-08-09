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

        // 1. 扫描所有.dll
        // 2. 加载这些.dll, 此时：
        //  1. ModuleMain如果想被加载上的话，无条件会有RegisterRiftModule。
        //  2. 因为2.1中已经在ModuleRegistry里注册了该Module，换言之此时已经有OnLoad, OnAllLoaded, OnUnload的函数指针。
        //  3. 保存Library和RiftModule信息，等待接下来调用。
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
        let binding = PathBuf::from(path).join("modules");
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
