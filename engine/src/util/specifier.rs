use std::path::Path;

use deno_core::ModuleSpecifier;

use super::errors::RiftResult;

pub fn specifier_from_file_path(path: &Path) -> RiftResult<ModuleSpecifier> {
    ModuleSpecifier::from_file_path(path)
        .map_err(|_| anyhow::anyhow!("Failed to create ModuleSpecifier from file path"))
}

/// 收集所有需要运行的.ts/.js/.jsx/...文件
pub fn collect_specifiers() {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_module_specifer() {
        type AstModuleSpecifier = deno_ast::ModuleSpecifier;
        let specifier = AstModuleSpecifier::parse("rift:!");
        println!("{:?}", specifier)
    }
}
