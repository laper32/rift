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
 * 综合下来的唯一解决方案还是做Manifest, aka: Rift.toml
 *
 * [module]
 * name = ""
 * version = ""
 * description = ""
 * url = ""
 * authors = []
 *
 * // 这里填加载所需的依赖，和版本，我们在这里找。
 * [dependencies]
 *
 */
use crate::{dir::PathIdentity, ffi::RiftModule};
use std::{fs, path::PathBuf};

pub const MODULE_DIR_NAME: &str = "modules";
const MODULE_ENTRY_FN: &[u8] = b"ModuleMain";

#[static_init::dynamic(drop)]
static mut INSTANCE: ModuleManager = ModuleManager::new();

pub fn init() {
    INSTANCE.write().init();
    println!("module::init()");
}

pub fn shutdown() {
    INSTANCE.write().shutdown();
    println!("module::shutdown()");
}

pub struct ModuleManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub authors: Vec<String>,
    // 入口dll，默认在bin/{文件夹同名}，除非你手动指定。
    pub entry: String,
}

pub struct ModuleManager {
    // modules: Vec<ModuleInstance>,
}

impl ModuleManager {
    fn new() -> Self {
        let installation_dir = PathIdentity::get_rift_path(PathIdentity::Installation);
        let user_profile_dir = PathIdentity::get_rift_path(PathIdentity::UserProfile);
        let project_dir = PathIdentity::get_rift_path(PathIdentity::Project);

        let mut possible_modules: Vec<PathBuf> = Vec::new();
        possible_modules.append(&mut Self::scan_modules(installation_dir));
        possible_modules.append(&mut Self::scan_modules(user_profile_dir));
        possible_modules.append(&mut Self::scan_modules(project_dir));

        Self {}
    }

    pub fn init(&mut self) {}

    pub fn shutdown(&self) {
        // for module in self.modules {
        //     module.shutdown();
        // }
    }

    fn scan_modules(search_path: String) -> Vec<PathBuf> {
        let mut possible_modules = Vec::new();
        let binding = PathBuf::from(search_path).join(MODULE_DIR_NAME);
        let path = binding.to_str().unwrap();
        let paths = fs::read_dir(path);
        match paths {
            Ok(paths) => {
                for path in paths {
                    match path {
                        Ok(path) => {
                            let path = path.path();
                            if Self::has_manifest(path.clone()) {
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
        possible_modules
    }

    fn has_manifest(path: PathBuf) -> bool {
        let manifest_path = path.join("Rift.toml");
        manifest_path.exists()
    }

    // fn load_module(module_path: &PathBuf) {
    //     let dll = unsafe { libloading::Library::new(module_path) };
    //     match dll {
    //         Ok(dll) => {
    //             let module_main_fn =
    //                 unsafe { dll.get::<unsafe extern "C" fn() -> i32>(MODULE_ENTRY_FN) };
    //             match module_main_fn {
    //                 Ok(module_main_fn) => {
    //                     let ret = unsafe { module_main_fn() };
    //                     if ret != 0 {
    //                         println!("Error on loading"); // TODO: LogError => Failed loading.
    //                         return;
    //                     }

    //                     println!("ModuleMain: {}", ret);
    //                 }
    //                 Err(e) => {
    //                     println!("FindFunc Error: {:?}", e);
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             println!("LoadLibrary Error: {:?}", e);
    //         }
    //     }
    // }

    // fn is_module(path: PathBuf) -> bool {
    //     #[cfg(target_os = "windows")]
    //     {
    //         let module_ext = "dll";
    //         match path.extension() {
    //             Some(ext) => ext == module_ext,
    //             None => false,
    //         }
    //     }
    //     #[cfg(target_os = "linux")]
    //     {
    //         let module_ext = "so";
    //         match path.extension() {
    //             Some(ext) => ext == module_ext,
    //             None => false,
    //         }
    //     }
    // }
}

// used for FFI
pub struct ModuleRegistry {
    modules: Vec<RiftModule>,
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

    pub fn register_module(&mut self, module: RiftModule) {
        self.modules.push(module);
    }
}
