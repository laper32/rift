// Rift PackageReference 完整示例
// 展示从简单到复杂的各种依赖声明形式

// ============================================
// Level 1: 最简单 - 工作空间本地依赖
// ============================================
Dependencies.add("utils");
Dependencies.add("core");

// ============================================
// Level 2: 带版本的基本形式
// ============================================
Dependencies.add("lodash@4.17.21");
Dependencies.add("react@18.2.0");

// ============================================
// Level 3: 对象形式
// ============================================
Dependencies.add({
    name: "typescript",
    version: "5.0.0"
});

// ============================================
// Level 4: npm 风格作用域包 (/ 分隔)
// ============================================
Dependencies.add("angular/core@16.0.0");
Dependencies.add("angular/common@16.0.0");
Dependencies.add("vue/router@4.2.0");
Dependencies.add("@babel/core@7.22.0");

// ============================================
// Level 5: Java/Maven 风格命名空间 (. 分隔)
// ============================================
Dependencies.add("org.apache.commons");
Dependencies.add("com.google.guava@31.1.0");
Dependencies.add("io.netty.netty-all@4.1.80");

// ============================================
// Level 6: 仓库路径风格
// ============================================
Dependencies.add("github.com/user/repo@v1.0.0");
Dependencies.add("gitlab.com/group/project@v2.0.0");

// ============================================
// Level 7: 多级命名空间
// ============================================
Dependencies.add("org.example.lib.core");
Dependencies.add("com.company.project.module@1.0.0");

// ============================================
// Level 8: 批量添加（数组形式）
// ============================================
Dependencies.add([
    "lodash@4.17.21",
    "react@18.2.0",
    "angular/core@16.0.0",
    { name: "typescript", version: "5.0.0" }
]);

// ============================================
// 实用示例：使用 PackageReferenceHelpers
// ============================================
// 如果需要解析或操作包引用
const ref = "angular/core@16.0.0";

// 注意：这些工具函数在插件或高级场景中更有用
// 在普通 dependencies.ts 中通常不需要

console.log("Dependencies configured using PackageReference");
