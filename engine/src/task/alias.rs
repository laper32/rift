/// Alias
/// 本质上来说是用于服务Task的。
/// Alias作为命令的简写包装，旨在于简化操作，如rift br可以展开成rift build --release
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::util::errors::RiftResult;

type TomlAlias = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
struct TomlAliasManifest {
    alias: TomlAlias,
}

pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<AliasManager> =
            once_cell::sync::Lazy::new(|| AliasManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn find_alias(&self, name: String) -> Option<&String> {
        self.aliases.get(&name)
    }

    pub fn insert_alias(&mut self, name: String, alias: String) -> RiftResult<()> {
        if self.has_alias(&name) {
            anyhow::bail!("Alias {} already exists", name)
        }
        self.aliases.insert(name, alias);
        Ok(())
    }

    pub fn has_alias(&self, name: &String) -> bool {
        self.aliases.contains_key(name)
    }

    pub fn remove_alias(&mut self, name: String) {
        self.aliases.remove(&name);
    }

    pub fn load_alias(&mut self, path: PathBuf) {
        let data = std::fs::read_to_string(path).unwrap();
        self.load_alias_from_string(data);
    }

    pub fn load_alias_from_string(&mut self, data: String) {
        let manifest = toml::from_str::<TomlAliasManifest>(&data);
        match manifest {
            Ok(manifest) => {
                for (alias_name, alias) in manifest.alias {
                    self.aliases.insert(alias_name, alias);
                }
            }
            Err(e) => {
                eprintln!("Error loading alias manifest: {}", e);
            }
        }
    }

    pub fn to_command_vec(&self, cmd: &str) -> RiftResult<Option<Vec<String>>> {
        let alias = Self::instance().find_alias(String::from(cmd));
        match alias {
            Some(_) => {
                let alias = alias.unwrap();
                let cmd_vec = alias
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                Ok(Some(cmd_vec))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {

    use super::AliasManager;

    #[test]
    fn test_load() {
        let raw = r#"
        [alias]
        rr = "build --release"
        "#;
        AliasManager::instance().load_alias_from_string(raw.to_owned());
        let cmd = AliasManager::instance().to_command_vec("rr");
        println!("{:?}", cmd);

        // println!("{:?}", toml::from_str::<TomlAliasManifest>(raw));
    }
}
