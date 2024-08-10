use serde::Deserialize;

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
use std::{collections::HashMap, fs, path::PathBuf};

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

// #[derive(Deserialize)]
#[derive(Debug)]
pub struct ModuleManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub authors: Vec<String>,
    // 入口dll，默认在bin/{文件夹同名}，除非你手动指定。
    pub entry: String,
    pub dependencies: HashMap<String, String>,
    pub metadata: HashMap<String, toml::Value>,
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

        // Step 1: 扫描所有可能的Module（Identifier: Rift.toml）
        possible_modules.append(&mut Self::scan_modules(installation_dir));
        possible_modules.append(&mut Self::scan_modules(user_profile_dir));
        possible_modules.append(&mut Self::scan_modules(project_dir));
        let mut manifests: Vec<ModuleManifest> = Vec::new();

        possible_modules.iter().for_each(|path| {
            let content = std::fs::read_to_string(path);
            match content {
                Ok(content) => {
                    let module_path = path.parent().unwrap().file_name().unwrap();
                    println!("Path: {module_path:?}");
                    let overall = content.parse::<toml::Table>().unwrap();
                    let module_table = overall.get("module").unwrap().as_table().unwrap();
                    let module_name = module_table.get("name").unwrap().as_str().unwrap();
                    let module_version = module_table.get("version").unwrap().as_str().unwrap();
                    let module_description =
                        module_table.get("description").unwrap().as_str().unwrap();
                    let module_url = module_table.get("url").unwrap().as_str().unwrap();
                    let module_authors = module_table.get("authors").unwrap().as_array().unwrap();
                    let authors: Vec<String> = module_authors
                        .iter()
                        .map(|author| author.as_str().unwrap().to_string())
                        .collect();

                    // bin/dir_name.dll
                    let default_entry = toml::Value::String(module_name.to_string());
                    let module_entry = module_table
                        .get("entry")
                        .unwrap_or(&default_entry)
                        .as_str()
                        .unwrap();

                    let mut dependencies: HashMap<String, String> = HashMap::new();
                    let mut metadata: HashMap<String, toml::Value> = HashMap::new();

                    let module_dependencies = overall.get("dependencies");
                    if module_dependencies.is_some() {
                        let module_dependencies = module_dependencies.unwrap().as_table().unwrap();
                        module_dependencies.iter().for_each(|(key, value)| {
                            dependencies
                                .insert(key.to_string(), value.as_str().unwrap().to_string());
                        });
                    }

                    let module_metadata = overall.get("metadata");
                    if module_metadata.is_some() {
                        let module_metadata = module_metadata.unwrap().as_table().unwrap();
                        module_metadata.iter().for_each(|(key, value)| {
                            metadata.insert(key.to_string(), value.clone());
                        });
                    }
                    let manifest = ModuleManifest {
                        name: module_name.to_string(),
                        version: module_version.to_string(),
                        description: module_description.to_string(),
                        url: module_url.to_string(),
                        authors,
                        entry: module_entry.to_string(),
                        dependencies,
                        metadata,
                    };
                    manifests.push(manifest);
                }
                Err(e) => {
                    println!("Failed to load \"{:?}\", Error: {:?}", path, e);
                }
            }
        });

        manifests.iter().for_each(|manifest| {
            println!("{:?}", manifest);
        });

        Self {}
    }

    pub fn init(&mut self) {}

    pub fn shutdown(&self) {
        // for module in self.modules {
        //     module.shutdown();
        // }
    }

    fn parse_module_manifest(manifest_path: String) {}

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
                                possible_modules.push(path.join("Rift.toml"));
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
