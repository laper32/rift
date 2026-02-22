# Rift Import Protocol (`rift:`) 设计文档

## 概述

`rift:` 协议用于 workspace 内部的包导入，允许一个包导出函数和常量，供其他包使用。这与 npm 包导入不同，`rift:` 专注于 workspace 内部的代码共享。

## 设计目标

1. **Workspace 内部导入**：允许 workspace 中的包（包括插件）导出和导入代码
2. **保持函数引用**：导出的函数保持其可调用性，不被序列化
3. **类型安全**：支持 TypeScript 类型检查
4. **简单 API**：提供简洁的导出/导入 API

## API 设计

### 导出 API

插件或包使用 `rift.registerExports()` 来导出函数和常量：

```typescript
// plugins/mathPlugin/index.ts
rift.registerExports("mathPlugin", {
    add: (x: number, y: number): number => {
        return x + y;
    },
    multiply: (x: number, y: number): number => {
        return x * y;
    },
    PI: 3.14159,
    E: 2.71828
});
```

### 导入 API（两种方式）

#### 方式 1：ES Module 语法（推荐）✨

使用标准的 ES Module `import` 语法：

```typescript
// app/tasks.ts
import { add, multiply, PI } from "rift:mathPlugin";

// 直接使用导入的函数
const result1 = add(5, 3);      // 8
const result2 = multiply(4, 7); // 28
console.log("PI constant:", PI); // 3.14159
```

#### 方式 2：API 函数调用

使用 `rift.getExports()` 函数：

```typescript
// app/tasks.ts
const mathPlugin = rift.getExports("mathPlugin");

// 使用导出的函数
const result1 = mathPlugin.add(5, 3);      // 8
const result2 = mathPlugin.multiply(4, 7); // 28
console.log("PI constant:", mathPlugin.PI); // 3.14159
```

**注意**：ES Module 语法会被自动转换成 `rift.getExports()` 调用，两者效果相同。

## 实现细节

### 核心组件

1. **RIFT_API_JS** ([src/vm/mod.rs](src/vm/mod.rs))
   - 添加了 `globalThis._rift_exports` 存储来保存导出内容
   - 添加了 `registerExports(packageName, exports)` 函数
   - 添加了 `getExports(packageName)` 函数
   - 创建了 `rift` 对象提供更好的 API 组织

2. **VM 修改** ([src/vm/mod.rs](src/vm/mod.rs))
   - 添加了 `resolve_workspace_package_import()` 函数处理 `rift:` 导入
   - 更新了 `collect_module_recursive()` 接受并传递 `all_packages` 参数
   - 添加了 `set_workspace_packages()` 方法配置 workspace 包

3. **ScriptResult 扩展** ([src/workspace/executor.rs](src/workspace/executor.rs))
   - 添加了 `exports: HashMap<String, String>` 字段存储包导出
   - 修改了 `execute_package()` 收集插件结果中的导出
   - 修复了早期返回以包含收集的导出

4. **任务执行修复** ([src/main.rs](src/main.rs))
   - 修改了 `execute_task_action()` 使用 VM 的 runtime 而不是创建新的
   - 确保插件加载时导出的函数在任务执行时仍然可用
   - 移除了重复的任务脚本执行

### 关键设计决策

#### 1. 不使用 JSON 序列化

最初尝试使用 `JSON.stringify()` 序列化导出内容，但这会导致函数丢失：

```typescript
// ❌ 不工作 - 函数被序列化后丢失
const exportsJson = JSON.stringify(exports);
ops.op_rift_register_exports(packageName, exportsJson);
```

**解决方案**：直接在 JavaScript 内存中保存导出内容，不进行序列化：

```typescript
// ✅ 工作 - 函数保持可调用
globalThis._rift_exports[packageName] = exports;
```

#### 2. 共享 Runtime

插件加载和任务执行需要使用同一个 JavaScript runtime，否则导出的函数在任务执行时不可用。

**解决方案**：使用 VM 的共享 runtime：

```rust
let mut runtime = vm.get_or_create_runtime()?;
```

#### 3. 避免重复执行

任务脚本在 `execute_scripts` 阶段已经执行过，任务执行时不应该再次执行，否则会导致 "Identifier already declared" 错误。

**解决方案**：移除任务执行时对任务脚本的重复执行。

#### 4. ES Module 语法自动转换 ✨

为了提供更好的开发体验，系统会自动将 `rift:` 导入语法转换为可执行的代码：

```typescript
// 用户代码
import { add, multiply } from "rift:mathPlugin";

// 自动转换为
const { add, multiply } = rift.getExports("mathPlugin");
```

转换在 `transpile_typescript_to_javascript` 函数中完成，通过 `process_rift_imports()` 函数实现。这允许开发者使用标准的 ES Module 语法，同时保持与现有运行时的兼容性。

## 使用示例

### 基本示例

见 [examples/rift-import-test/](../examples/rift-import-test/)：

```
examples/rift-import-test/
├── Rift.toml              # Workspace 配置
├── plugins/
│   └── mathPlugin/
│       ├── Rift.toml      # Plugin 配置
│       └── index.ts       # 导出数学函数
└── app/
    ├── Rift.toml          # Project 配置
    └── tasks.ts           # 使用导出的函数
```

### 运行测试

```bash
cd examples/rift-import-test
cargo run --bin rift -- test-math
```

**输出**：
```
[app] Testing rift: import functionality...
[app] add(5, 3) = 8
[app] multiply(4, 7) = 28
[app] PI constant: 3.14159
[app] SUCCESS: All imports work correctly!
```

## 与其他系统的对比

### vs Brioche

Brioche 使用双层导入系统：
- 构建配置层：`import * as std from "std"` (内置模块)
- 应用代码：`import * as esbuild from "esbuild"` (npm 包)

Rift 使用类似的分层，但专注于 workspace 内部：
- Workspace 包：`import { add } from "rift:mathPlugin"` (通过 `rift.getExports()`)
- 未来支持：Rift 自己的包注册表（类似 NuGet）

### vs Puerts (Unreal Engine)

Puerts 直接使用 npm 和 `package.json`。

Rift 当前不支持 npm/pnpm install，原因：
- Rift 是构建系统，专注于构建任务
- npm 依赖管理在构建系统场景下过于复杂

**未来考虑**：为了兼容现有生态，可能会考虑支持 npm 包，但这需要权衡复杂度和收益。

## 未来扩展

1. **Rift 包注册表**：实现类似 NuGet 的中央包仓库
2. **类型导出**：支持 TypeScript 类型定义的导出，提供更好的类型安全
3. **版本管理**：支持包版本约束和解析
4. **npm 生态兼容性**（待定）：考虑与 npm 生态的兼容性
   - **当前状态**：暂时不支持 npm/pnpm install，因为：
     - Rift 是构建系统，不是前端开发工具
     - npm 依赖管理过于复杂
   - **未来考虑**：为了兼容现有生态，可能会考虑直接复用 npm 包
     - 但需要权衡：构建系统场景下的复杂度 vs 生态兼容性
     - 可以参考 Puerts（Unreal Engine）的 TypeScript 集成方案

## 相关文件

- [src/vm/mod.rs](../../src/vm/mod.rs) - VM 实现，包含 RIFT_API_JS
- [src/workspace/executor.rs](../../src/workspace/executor.rs) - 执行器，处理脚本加载
- [src/main.rs](../../src/main.rs) - 主入口，处理任务执行
- [examples/rift-import-test/](../../examples/rift-import-test/) - 测试示例

## 参考资料

- [Task System Design](./task-system-design.md) - 任务系统设计
- [Plugin Registry Implementation](./plugin-registry-implementation.md) - 插件注册表实现
