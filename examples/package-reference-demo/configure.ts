// Rift PackageReference configure 示例
// 展示插件导出的 API 如何与 PackageReference 配合

// ============================================
// 示例 1: Go 语言插件配置
// ============================================
// rift.go 插件导出的 API
go.setVersion("1.21.0");

// ============================================
// 示例 2: Rust 语言插件配置
// ============================================
// 假设 rift.rust 插件导出类似 API
// rust.setEdition("2021");
// rust.addDependency("serde@1.0.0");
// rust.addDependency("tokio@1.28.0", {
//     features: ["full", "macros"]
// });

// ============================================
// 示例 3: TypeScript/Node.js 插件配置
// ============================================
// 假设 rift.ts 插件导出 API
// ts.setVersion("5.0.0");
// ts.setModule("ESNext");

// ============================================
// 示例 4: 通用配置
// ============================================
// 使用 config API 设置任意配置
config.set("build.target", "es2020");
config.set("build.minify", true);
config.set("build.sourceMaps", true);

// ============================================
// 示例 5: 嵌套配置
// ============================================
config.set("compiler.options.strict", true);
config.set("compiler.options.jsx", "preserve");

// ============================================
// 示例 6: 插件特定配置的另一种模式
// ============================================
// 有些插件可能使用对象配置
// pkg.configure((config) => {
//     config.set("go.version", "1.21.0");
//     config.set("go.cgo", false);
//     config.set("go.ldflags", "-s -w");
// });

console.log("Package configured successfully");
