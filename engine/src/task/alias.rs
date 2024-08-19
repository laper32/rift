/*
[alias]
command = "actual execution command" eg: rift br => rift build --release
*/
pub struct TomlAlias {}

pub struct AliasManager {}

#[cfg(test)]
mod test {
    fn test_load() {
        let raw = r#"
        [alias]
        rr = "build --release"
        "#;
    }
}
