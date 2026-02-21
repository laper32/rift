// Rift PackageReference tasks 示例
// 展示任务定义如何与依赖系统配合

// ============================================
// 示例 1: 简单任务定义
// ============================================
tasks.register("build", {
    description: "Build the project",
    run: () => {
        console.log("Building project...");
        console.log("Dependencies would be resolved here");
    }
});

// ============================================
// 示例 2: 带依赖的任务
// ============================================
tasks.register("app.build", {
    description: "Build the application",
    dependsOn: ["clean", "deps.install"],
    command: true,  // 作为命令暴露
    run: () => {
        console.log("Building app after cleaning and installing deps");
    }
});

// ============================================
// 示例 3: 多级任务
// ============================================
tasks.register("app.build.dev", {
    description: "Build for development",
    dependsOn: ["app.build"],
    run: () => {
        console.log("Building app in dev mode");
    }
});

tasks.register("app.build.prod", {
    description: "Build for production",
    dependsOn: ["app.build"],
    run: () => {
        console.log("Building app in prod mode");
    }
});

// ============================================
// 示例 4: 使用旧版 builder API（向后兼容）
// ============================================
tasks.register("test", (task) => {
    task.setDescription("Run tests");
    task.dependsOn("app.build");
    task.asCommand(true);
    task.do(() => {
        console.log("Running tests...");
    });
});

// ============================================
// 示例 5: 跨包任务依赖
// ============================================
// 依赖另一个包的任务
tasks.register("app.deploy", {
    description: "Deploy the application",
    dependsOn: ["app.build.prod", "utils.build", "core.build"],
    command: true,
    run: () => {
        console.log("Deploying app with all dependencies built");
    }
});

// ============================================
// 示例 6: 依赖外部包的任务
// ============================================
tasks.register("generate.code", {
    description: "Generate code from external tools",
    dependsOn: ["protoc.build"],  // 假设 protoc 是外部依赖
    run: () => {
        console.log("Generating code from protobuf definitions");
    }
});

console.log("Tasks registered successfully");
