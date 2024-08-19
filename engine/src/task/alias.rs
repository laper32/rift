// Task的一部分。
// 方便维护所以拆成了两个Manager。

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::RiftResult;

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

    pub fn find_alias(&self, name: String) -> String {
        self.aliases.get(&name).unwrap().to_string()
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
}

#[cfg(test)]
mod test {
    use crate::task::alias::TomlAliasManifest;

    #[test]
    fn test_load() {
        let raw = r#"
        [alias]
        rr = "build --release"
        "#;

        println!("{:?}", toml::from_str::<TomlAliasManifest>(raw));
    }
}
