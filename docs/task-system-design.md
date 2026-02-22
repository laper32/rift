# Task System Design

## 设计参考

参考 Gradle (Java) / Cake (C#) / Make 的任务定义方式。

---

## Makefile 的 Task 定义

### 基本语法

```makefile
target: dependencies
	command
```

### 具体例子

当用户执行 `make abc` 时，Makefile 如何定义这个 abc task：

```makefile
# 最简单的定义
abc:
	echo "running abc"

# 带依赖的定义
abc: def
	echo "running abc after def"

# 多个命令
abc:
	step1
	step2
	step3

# 多个依赖
abc: def ghi
	echo "running abc after def and ghi"
```

### 执行流程

```
用户执行: make abc
    ↓
Make 查找名为 "abc" 的 target
    ↓
如果 abc 有依赖 (def, ghi)
    ↓
    先执行 def
    再执行 ghi
    ↓
最后执行 abc 的命令
```

### 完整示例

```makefile
# Makefile

build:
	cargo build

test: build
	cargo test

run: build
	./target/debug/myapp

clean:
	rm -rf target

.PHONY: build test run clean
```

```
$ make test
cargo build
cargo test
```

### 关键点

1. **命名** - target 名字就是用户输入的命令名
2. **依赖** - 冒号后列出前置 targets
3. **命令** - tab 开头的 shell 命令
4. **查找** - Make 自动查找匹配的 target

---

## Gradle 的 Task 定义

### 基本语法

```gradle
task taskName {
    // 配置
    dependsOn 'otherTask'
    doLast {
        // 执行逻辑
    }
}
```

### 具体例子

```gradle
// build.gradle

task build {
    doLast {
        println 'Building...'
    }
}

task test(dependsOn: 'build') {
    doLast {
        println 'Testing...'
    }
}

task clean {
    doLast {
        delete 'build'
    }
}
```

### 执行流程

```
用户执行: ./gradlew test
    ↓
Gradle 查找名为 "test" 的 task
    ↓
解析依赖关系 (test → build)
    ↓
执行 build 的 doLast
    ↓
执行 test 的 doLast
```

---

## 对比

| 特性 | Make | Gradle |
|------|------|--------|
| 定义位置 | Makefile | build.gradle |
| 依赖语法 | `target: deps` | `dependsOn: 'deps'` |
| 执行内容 | shell 命令 | Groovy 闭包 |
| 查找方式 | 按 target 名称 | 按 task 名称 |

---

## Task 的来源

参考 Gradle 和 CMake，task 可以有多个来源：

### 1. 插件定义的 Task

**Gradle 例子：**
```gradle
// java-gradle-plugin 插件内部
class JavaPlugin implements Plugin<Project> {
    void apply(Project project) {
        project.tasks.register('build', BuildTask)
        project.tasks.register('test', TestTask)
        project.tasks.register('jar', JarTask)
    }
}

// 项目中使用
plugins {
    id 'java'
}
// build, test, jar 任务自动可用
```

**Rift 的对应需求：**
```typescript
// 在插件中 (如 rust.plugin)
defineTasks((tasks) => {
    tasks.register("build", {
        description: "Build with Cargo",
        run: (ctx) => {
            // 不仅仅是执行命令
            exec("cargo", ["build"]);

            // 可以做任何 TypeScript 能做的事：
            // - 读写文件
            // - 复杂逻辑判断
            // - 并行处理
            // - 调用其他 API
        }
    });

    tasks.register("test", {
        description: "Run tests",
        dependsOn: ["build"],
        run: (ctx) => {
            exec("cargo", ["test"]);
        }
    });
});
```

### 2. 项目预定义的 Task

**Gradle 例子：**
```gradle
// build.gradle (项目自己定义)
task myCustomTask {
    doLast {
        println 'Custom task'
    }
}
```

**Rift 的对应需求：**
```typescript
// 项目的 tasks.ts
tasks.register("myTask", {
    description: "My custom task",
    run: (ctx) => { ... }
});
```

### 3. 团队/组织特定的 Task

**重要：不是 Phony Project，而是插件！**

如果团队/组织有特定需求，应该写成插件：

```typescript
// company-tools.plugin/index.ts
// 公司/团队的特定工具任务
defineTasks((tasks) => {
    tasks.register("deploy-staging", {
        description: "Deploy to staging environment",
        run: (ctx) => {
            // 公司特定的部署流程
            exec("./scripts/deploy-staging.sh");
        }
    });

    tasks.register("deploy-prod", {
        description: "Deploy to production",
        run: (ctx) => {
            // 公司特定的生产部署流程
            exec("./scripts/deploy-prod.sh");
        }
    });

    tasks.register("sync-translations", {
        description: "Sync translation files",
        run: (ctx) => {
            // 公司特定的翻译同步流程
            exec("./scripts/sync-i18n.sh");
        }
    });
});
```

**为什么用插件而不是 phony project？**

| 对比 | 插件 | Phony project |
|------|------|---------------|
| 可复用性 | ✅ 多个项目共享 | ❌ 每个项目都要引入 |
| 版本管理 | ✅ 独立版本和发布 | ❌ 绑定在项目中 |
| 依赖管理 | ✅ 插件系统管理 | ❌ 手动管理 |
| 分发 | ✅ 发布到 registry | ❌ 只能复制代码 |

---

## Task 查找和合并策略

当用户执行 `rift abc` 时：

```
1. 搜索所有 task 来源：
   - 所有插件注册的 tasks
   - 当前项目的 tasks.ts

2. 按优先级合并（后者覆盖前者）：
   插件 tasks ← 项目 tasks

3. 解析依赖关系

4. 执行
```

### 优先级规则

| 来源 | 优先级 | 说明 |
|------|--------|------|
| 插件 | 高 | 语言/框架/团队定义的标准任务 |
| 项目 tasks.ts | **可覆盖** | 项目自定义，可覆盖插件定义 |

**示例：**

```typescript
// typescript.plugin 定义标准 build
tasks.register("build", {
    run: (ctx) => exec("tsc", ["-p", "tsconfig.json"])
});

// 项目 tasks.ts 覆盖（如果需要自定义）
tasks.register("build", {
    run: (ctx) => {
        // 自定义构建流程
        exec("tsc", ["-p", "tsconfig.custom.json"]);
        exec("webpack", ["--mode", "production"]);
    }
});

// ✅ 项目的版本覆盖插件
```

---

## Task 注册的约束

### 重要：Task 不能随意注册

**参考 Gradle：** 只有特定位置才能注册 task，不是任何 `.kts` 文件都可以。

| 位置 | 能否注册 Task | 说明 |
|------|--------------|------|
| `build.gradle.kts` | ✅ | 项目主要构建脚本 |
| 插件代码 | ✅ | 通过 `Plugin.apply()` |
| `init.gradle.kts` | ✅ | 全局初始化脚本 |
| 任意 `.kts` 文件 | ❌ | 不会被加载 |

**Rift 遵循相同原则：**

| 位置 | 能否注册 Task | 说明 |
|------|--------------|------|
| `tasks.ts` | ✅ | 项目任务定义 |
| 插件 `index.ts` | ✅ | 插件注册任务 |
| Phony project `tasks.ts` | ✅ | 工具任务定义 |
| 任意 `.ts` 文件 | ❌ | 不会被加载 |

### 原因

1. **可预测性** - 开发者清楚在哪里找到任务定义
2. **避免混乱** - 防止任务散落在各种文件中
3. **加载效率** - 只需扫描特定文件
4. **维护性** - 集中管理，易于理解和修改

---

## Rift 的设计问题

### ~~Q1: 插件如何注册 task？~~

**✅ 已解决** - 插件在 `index.ts` 中直接调用全局 API 即可。详见 [待解决的核心问题](#待解决的核心问题-open-questions) 章节。

### Q2: VM Scope 设计

**核心问题：不是简单的"VM 保持状态"，而是 VM Scope（作用域）架构**

#### 场景：不同项目需要不同版本的插件

```
Workspace (Root Scope)
    ├─ Project A (虚幻5.0) → VM Scope A
    │   ├─ unreal.plugin@5.0
    │   └─ tasks: build, package
    │
    ├─ Project B (虚幻5.1) → VM Scope B
    │   ├─ unreal.plugin@5.1  (不同版本！)
    │   └─ tasks: build, package
    │
    └─ Project C (Rust) → VM Scope C
        ├─ rust.plugin
        └─ tasks: build, test
```

#### 关键问题

1. **每个 scope 独立 VM 还是共享 VM？**

| 方案 | 优点 | 缺点 |
|------|------|------|
| 独立 VM | 隔离性好，无版本冲突 | 内存开销大，跨 scope 依赖复杂 |
| 共享 VM | 内存效率高 | 版本冲突，需要命名空间隔离 |

2. **跨 scope 的 task 依赖如何处理？**

```typescript
// Project A 的 task 依赖 Project B 的 task
tasks.register("build-all", {
    dependsOn: ["projectB:build"],  // 跨 scope 依赖
    run: (ctx) => { ... }
});
```

3. **数据如何在 scope 间传递？**

```typescript
// Scope A 的构建产物，Scope B 需要使用
// 产物路径、环境变量等如何传递？
```

#### 插件声明是提前知道的

```toml
# Project A/Rift.toml
[dependencies]
unreal-engine = "5.0"  # 显式声明

# Project B/Rift.toml
[dependencies]
unreal-engine = "5.1"  # 不同版本
```

**优势：** 系统可以提前知道每个项目需要什么插件，从而：
- 提前构造对应 scope 的 VM
- 按需加载插件
- 优化内存使用

### ~~Q3: Phony project 如何工作？~~

**✅ 已解决：使用插件代替 phony project**

团队/组织特定需求应写成插件，参考 [团队/组织特定的 Task](#3-团队组织特定的-task) 章节。

---

## 待解决的核心问题 (Open Questions)

### ~~Q1: 插件如何注册 task？~~

**✅ 已解决：插件在 `index.ts` 中直接调用全局 API**

```typescript
// rust.plugin/index.ts
// Rift API 全局可用：tasks, defineCommand, emit, on 等
tasks.register("build", (task) => {
    task.description = "Build with Cargo";
    task.do(() => {
        exec("cargo", ["build"]);
    });
});
```

**工作原理：**
1. 插件的 `index.ts` 是执行入口
2. VM 加载插件时执行 `index.ts`
3. `tasks.register()` 等全局 API 在插件 context 中可用
4. 调用即注册，无需额外机制

**参考：** [examples/plugins/rift.generate/index.ts](../examples/plugins/rift.generate/index.ts)

### Q2: VM Scope 设计

**核心问题：不是简单的"VM 保持状态"，而是 VM Scope（作用域）架构**

#### 场景：不同项目需要不同版本的插件

```
Workspace (Root Scope)
    ├─ Project A (虚幻5.0) → VM Scope A
    │   ├─ unreal.plugin@5.0
    │   └─ tasks: build, package
    │
    ├─ Project B (虚幻5.1) → VM Scope B
    │   ├─ unreal.plugin@5.1  (不同版本！)
    │   └─ tasks: build, package
    │
    └─ Project C (Rust) → VM Scope C
        ├─ rust.plugin
        └─ tasks: build, test
```

#### 关键问题

1. **每个 scope 独立 VM 还是共享 VM？**

| 方案 | 优点 | 缺点 |
|------|------|------|
| 独立 VM | 隔离性好，无版本冲突 | 内存开销大，跨 scope 依赖复杂 |
| 共享 VM | 内存效率高 | 版本冲突，需要命名空间隔离 |

2. **跨 scope 的 task 依赖如何处理？**

```typescript
// Project A 的 task 依赖 Project B 的 task
tasks.register("build-all", {
    dependsOn: ["projectB:build"],  // 跨 scope 依赖
    run: (ctx) => { ... }
});
```

3. **数据如何在 scope 间传递？**

```typescript
// Scope A 的构建产物，Scope B 需要使用
// 产物路径、环境变量等如何传递？
```

#### 配置文件执行阶段问题

**关键发现：** dependencies.ts, plugins.ts, metadata.ts 都是 TypeScript，但执行时机不同！

```
Phase 1: metadata.ts     (最先执行，定义共享数据)
   └─ workspace/metadata.ts     → Workspace Scope
   └─ packages/*/metadata.ts     → Package Scope (读取 workspace metadata)

Phase 2: dependencies.ts (依赖解析)
   └─ workspace/dependencies.ts  → Workspace Scope
   └─ packages/*/dependencies.ts  → Package Scope

Phase 3: plugins.ts      (插件加载)
   └─ workspace/plugins.ts       → Workspace Scope
   └─ packages/*/plugins.ts       → Package Scope
```

**具体例子：条件依赖**

```typescript
// workspace/metadata.ts
config.defineVariable("CURRENT_COMPANY", "a");

// packages/myapp/dependencies.ts
let currentCompany = config.getVariable("CURRENT_COMPANY");
if (currentCompany === 'a') {
    addDependency({ name: "@company-a/internal-lib", version: "1.0.0" });
} else if (currentCompany === 'b') {
    addDependency({ name: "@company-b/internal-lib", version: "2.0.0" });
} else {
    addDependency({ name: "open-source-lib", version: "3.0.0" });
}
```

**这揭示了：**
1. **不是 ES import** - 通过 `config` API 进行运行时数据传递
2. **条件依赖** - dependencies.ts 可以根据 workspace 配置做动态决策
3. **全局状态** - `config` 是跨 scope 共享的全局对象
4. **执行顺序强制** - metadata.ts 必须先于 dependencies.ts 执行

**config API 设计：**

```typescript
// config 是注入到 VM context 的全局对象
interface ConfigRegistry {
    // 定义变量（workspace 级别）
    defineVariable(name: string, value: any): void;

    // 读取变量（package 级别可以读 workspace 的）
    getVariable(name: string): any;

    // 可选：命名空间隔离
    getVariable(name: string, scope?: "workspace" | "package"): any;
}
```

#### Event Bus 的染色问题

**核心问题：** 当前单一 VM 实例导致不同插件版本的 handlers 混在一起。

```javascript
// 当前实现：全局 handler 存储
globalThis._rift_event_handlers = {};

// 加载 rust.plugin@1.0
on("onBuild", handler_v1);
// _rift_event_handlers.onBuild = [handler_v1]

// 加载 rust.plugin@1.5
on("onBuild", handler_v2);
// _rift_event_handlers.onBuild = [handler_v1, handler_v2]
//                              ↑ 问题：两个版本的 handler 混在一起！
```

**染色维度：**

| 维度 | 目的 | 示例 |
|------|------|------|
| 插件版本 | 区分同一插件的不同版本 | `rust.plugin@1.0` vs `rust.plugin@1.5` |
| Scope 归属 | 区分不同 package 的 handlers | `packageA` vs `packageB` |
| 执行阶段 | 区分 metadata/dependencies/plugins | Phase 1/2/3 |

**染色后的 Event Handler 结构：**

```typescript
interface ColoredHandler {
    // 染色标识
    color: {
        plugin: string;      // "rust.plugin"
        version: string;     // "1.0.0"
        scope: string;       // "packageA"
        phase: number;       // 1/2/3
    };
    // 实际的 handler 函数
    handler: (context: any) => void;
}
```

**Event Bus 路由逻辑：**

```typescript
// 用户在 packageA/ 目录执行 `rift build`
// → 只执行 packageA scope 的 handlers
emit("onBuild", {
    scope: "packageA",  // 指定 scope
    context: { ... }
});

// 用户在 workspace root 执行 `rift build`
// → 执行所有 scope 的 handlers，按依赖顺序
emit("onBuild", {
    scope: "all",  // 或 undefined
    context: { ... }
});
```

**类比 sbox 的 Assembly Swap Pattern：**

| sbox | Rift |
|------|------|
| `Dictionary<Assembly, Assembly>` | `Map<ColorId, HandlerList>` |
| Assembly 作为染色单位 | Plugin@Scope 作为染色单位 |
| Hotload 时 swap colors | 执行时按 color 路由 |
| 更新所有引用到新 assembly | 隔离不同版本的 handlers |

#### 染色方案决策：组合字符串

**选择方案 1** - 组合字符串作为染色标识：
```typescript
const color = `${plugin}@${version}:${scope}`;
// "rust.plugin@1.0.0:packageA"
```

**理由：**
1. 实现最简单，bug 最少
2. 天然支持通配符匹配：`"rust.plugin@*:*"` 匹配所有版本和 scope
3. 调试友好，日志里直接可读
4. 内存占用小，序列化简单

**Color 格式：**
```
格式：{plugin}@{version}:{scope}
示例：
  - "rust.plugin@1.0.0:packageA"
  - "unreal.plugin@5.1:frontend"
  - "company-tools@2.0.0:workspace"

通配符匹配：
  - "rust.plugin@*:*"       → 所有版本，所有 scope
  - "rust.plugin@1.0.0:*"    → 1.0.0 版本，所有 scope
  - "*@*:packageA"          → packageA 的所有插件
```

**Event Handler 存储结构：**
```typescript
// 旧的（问题）：全局 handler 数组
_rift_event_handlers = {
    "onBuild": [handler_v1, handler_v2]  // 混在一起！
}

// 新的（染色）：每个 color 独立存储
_rift_event_handlers = {
    "onBuild": {
        "rust.plugin@1.0.0:packageA": [handler1, handler2],
        "rust.plugin@1.5.0:packageB": [handler3],
        "go.plugin@2.0.0:workspace": [handler4]
    }
}
```

**实现要点：**
```typescript
// 需要转义插件名中的特殊字符
function escapeColorPart(part: string): string {
    return part.replace(/[@:]/g, '\\$&');
}

// 注册 handler 时带上 color
function on(eventName: string, handler: Function, color: string) {
    _rift_event_handlers[eventName][color].push(handler);
}

// 触发时按 color 过滤
function emit(eventName: string, context: any, targetColor?: string) {
    const handlers = _rift_event_handlers[eventName];

    if (targetColor) {
        // 精确匹配
        handlers[targetColor]?.forEach(h => h(context));
    } else {
        // 执行所有 colors
        Object.values(handlers).forEach(list => list.forEach(h => h(context)));
    }
}
```

### Q3: 共享插件如何处理版本兼容？

**场景：** 公司自研插件 `company-c.plugin` 需要同时支持：
- `unreal.plugin@5.0` + `company-c.plugin`
- `unreal.plugin@5.1` + `company-c.plugin`

这类似 C# 的 Assembly Load Context (ALC) 问题：
- 同一个插件在不同上下文中可能依赖不同版本的依赖
- 如何确保 `company-c.plugin` 能在不同 scope 中正常工作？

**参考实现：** [sbox engine (Facepunch)](https://github.com/Facepunch/sbox-public)
- C# runtime layer with plugin/hot-reload mechanism
- 使用 AssemblyLoadContext 实现插件隔离

### Q4: Task 注册和查找的具体实现

1. **配置文件执行顺序** - metadata → dependencies → plugins，如何保证？
2. **跨 scope 数据可见** - package 如何读取 workspace 的 metadata？
3. **Task 合并策略** - 当多个插件定义同名 task 时，如何处理？
4. **Task 查找算法** - 用户执行 `rift taskname` 时，如何快速定位 task 定义？
5. **Scope 间通信** - 不同 VM scope 之间如何传递数据？

### Q5: VM 实例管理

1. **VM 生命周期** - VM 何时创建、销毁？
2. **状态持久化** - VM 中哪些状态需要持久化？
3. **热重载支持** - 代码变更时，如何重新加载 VM？
4. **内存优化** - 如何避免创建过多 VM 实例？

---

## 参考实现研究

### sbox Engine (Facepunch)

**仓库：** https://github.com/Facepunch/sbox-public

**关键特性：**
- 基于 .NET 10
- C# runtime layer
- 插件系统和热重载机制
- AssemblyLoadContext 实现插件隔离

**需要研究的内容：**
1. 如何实现 AssemblyLoadContext
2. 插件如何在不同上下文中加载
3. 热重载的实现机制
4. 插件间的通信机制

---

### sbox Hotload 系统详细分析

#### 1. 核心：Assembly Swap Pattern

**文件：** `engine/Sandbox.Hotload/Hotload.cs`

**关键数据结构：**
```csharp
/// 旧程序集到新程序集的映射
private readonly Dictionary<Assembly, Assembly> Swaps = new Dictionary<Assembly, Assembly>();

/// 当前 hotload 中正在加载的程序集
private readonly HashSet<Assembly> New = new HashSet<Assembly>();

/// 注册需要交换的程序集
public bool ReplacingAssembly(Assembly oldAssembly, Assembly newAssembly)
{
    Swaps[oldAssembly] = newAssembly;
    New.Add(newAssembly);
    // ... 处理交换链和依赖关系
}
```

**设计模式要点：**
- 使用 `Dictionary<Assembly, Assembly>` 跟踪旧→新版本映射
- 支持交换链：A→B, B→C 会被简化为 A→C
- 旧程序集仍在内存中，但所有引用会被重定向到新版本

#### 2. 引用更新机制

**文件：** `engine/Sandbox.Hotload/UpdateReferences.cs`

**关键算法：**
```csharp
public HotloadResult UpdateReferences()
{
    // 1. 简化交换链 (A→B, B→C => A→C)
    SimplifySwaps();

    // 2. 更新静态字段中的引用
    foreach (var (type, fields) in watchedAssemblies.SelectMany(GetWatchedFields))
    {
        UpdateReferencesInType(type, fields);
    }

    // 3. 更新被监视的实例
    foreach (var instance in WatchedInstances)
    {
        defaultUpgrader.ProcessObjectFields(instance);
    }

    // 4. 处理自定义升级器
    foreach (var upgrader in CustomUpgraders)
    {
        upgrader.UpgradeAll();
    }
}
```

**递归字段遍历：**
```csharp
private void UpdateReferencesInType(Type type, FieldInfo[] fields)
{
    foreach (var field in fields)
    {
        var value = field.GetValue(null);
        if (value is null) continue;

        // 如果字段类型需要交换，递归处理
        var newValue = SwapValue(value);
        if (newValue != value)
        {
            field.SetValue(null, newValue);
        }
    }
}
```

#### 3. 实例升级接口

**文件：** `engine/Sandbox.Hotload/InstanceUpgrader.cs`

```csharp
public interface IInstanceUpgrader
{
    // 判断是否处理该类型
    bool ShouldProcessType(Type type);

    // 创建新实例（对象重建模式）
    bool TryCreateNewInstance(object oldInstance, out object newInstance);

    // 升级实例（字段迁移模式）
    bool TryUpgradeInstance(object oldInstance, object newInstance);
}
```

**两种升级策略：**

| 策略 | 适用场景 | 实现 |
|------|---------|------|
| 对象重建 | 不可变对象、简单 DTO | `TryCreateNewInstance` - 创建新对象并复制字段 |
| 字段迁移 | 复杂对象、保持引用 | `TryUpgradeInstance` - 原地更新字段值 |

#### 4. 监视机制

**文件：** `engine/Sandbox.Hotload/Watch.cs`

```csharp
// 监视程序集中的所有类型
public void WatchAssembly(Assembly a, Func<Type, bool> filter = null)
{
    watchedAssemblies.Add(a);
    // ... 注册需要监视的类型
}

// 监视特定实例
public void WatchInstance<T>(T obj) where T : class
{
    WatchedInstances.Add(obj);
}
```

**监视范围：**
- 静态字段（通过程序集监视）
- 实例字段（通过实例监视）
- 支持类型过滤器（避免监视不需要的类型）

---

### sbox → Rift 的设计映射

#### 问题对照表

| sbox 问题 | Rift 对应问题 | sbox 解决方案 | Rift 可借鉴方案 |
|----------|--------------|--------------|----------------|
| 不同版本的程序集 | 不同版本的插件 | AssemblyLoadContext 隔离 | Deno VM Context 隔离 |
| 热重载时引用失效 | 跨 scope task 依赖 | Assembly Swap Dictionary | Plugin Registry Map |
| 对象状态迁移 | 跨 scope 数据传递 | InstanceUpgrader 接口 | Context Bridge API |
| 类型查找 | Task 查找 | Type Substitution | Task Resolution by Scope |

#### Rift VM Scope 架构建议

**基于 sbox 的模式，建议 Rift 采用：**

```
Workspace (Root)
├── ScopeManager
│   ├── scopes: Map<PackageId, VMScope>
│   └── pluginRegistry: Map<PluginName, Map<Version, PluginInstance>>
│
├── VMScope A (Project A)
│   ├── vm: Deno.Runtime (isolated)
│   ├── plugins: [unreal@5.0, company-c@2.0]
│   └── tasks: build, package
│
├── VMScope B (Project B)
│   ├── vm: Deno.Runtime (isolated)
│   ├── plugins: [unreal@5.1, company-c@2.0]
│   └── tasks: build, package
│
└── ScopeBridge (跨 scope 通信)
    ├── resolveTask(scope, taskName) -> TaskHandle
    └── transferData(fromScope, toScope, data)
```

**关键设计决策（基于 sbox 经验）：**

1. **隔离优先** - 每个项目独立的 VM Context（类似 ALC）
2. **共享插件策略** - company-c.plugin 在不同 scope 中加载不同实例
3. **Task 引用** - 使用 `project:task` 语法跨 scope 引用（类似 sbox 的类型查找）
4. **状态传递** - 通过显式的 Bridge API（类似 InstanceUpgrader 的字段迁移）

**TODO:**
- [x] 分析 sbox 的 Runtime layer 实现
- [x] 研究 ALC 的具体使用方式
- [x] 理解插件隔离和版本管理策略
- [x] 提取可应用到 Rift 的设计模式
- [x] 实现染色 Event Bus
- [x] 验证多版本插件隔离
- [ ] 实现 VMScope 和 ScopeManager

---

## 实现总结：染色 Event Bus

### 实现日期
2026-02-22

### 方案选择
**方案 1：组合字符串作为染色标识**
- 格式：`{plugin}@{version}:{scope}`
- 例如：`rust.plugin@1.0.0:packageA`

### 核心实现

#### 1. JavaScript API (RIFT_API_JS)

**颜色上下文注入：**
```javascript
// 在插件加载时设置颜色
globalThis._rift_current_color = "rust.plugin@1.0.0:packageA";
```

**Event Handler 存储：**
```javascript
// 染色后的存储结构
_rift_event_handlers = {
    "onBuild": {
        "rust.plugin@1.0.0:packageA": [handler1, handler2],
        "rust.plugin@1.5.0:packageB": [handler3]
    }
}

// 注册时带上当前颜色
const on = (eventName, handler) => {
    const color = globalThis._rift_current_color || 'unknown:unknown:unknown';
    ops.op_rift_on(eventName, color);
    // ...
};
```

**Emit 的默认隔离行为：**
```javascript
const emit = (eventName, context, options) => {
    const currentColor = globalThis._rift_current_color || 'unknown:unknown:unknown';
    const targetScope = options?.scope;
    const targetColor = options?.color;

    for (const [color, handlers] of Object.entries(eventHandlers)) {
        // 默认：只执行当前颜色的 handlers（隔离）
        const useDefaultFilter = !targetScope && !targetColor;

        if (useDefaultFilter) {
            if (color !== currentColor) continue;  // 跳过其他颜色的 handlers
        }

        // scope: "all" 跳过 scope 过滤，执行所有
        if (targetScope && targetScope !== 'all') {
            const colorScope = color.split(':')[1] || '';
            if (colorScope !== targetScope && colorScope !== 'workspace') {
                continue;
            }
        }

        // 执行 handlers
        for (const handler of handlers) {
            handler(context);
        }
    }
};
```

#### 2. Rust 结构更新

**ScriptContext 新增字段：**
```rust
pub struct ScriptContext {
    // ... 原有字段
    pub plugin_name: Option<String>,
    pub plugin_version: Option<String>,
    pub color_scope: String,
}
```

**事件订阅改为带颜色：**
```rust
pub struct RiftOpState {
    pub event_subscriptions: Vec<(String, String)>,  // (event_name, color)
}

#[op2(fast)]
fn op_rift_on(state: &mut OpState, #[string] event_name: String, #[string] color: String) {
    // ...
}
```

#### 3. 命令执行颜色保留

**main.rs 中的修复：**
```rust
// 加载每个插件前设置颜色
for (path_str, ts_content, plugin_name, plugin_version) in plugin_scripts {
    let color = format!("{}@{}:{}", plugin_name, plugin_version, plugin_name);
    let color_script = format!("globalThis._rift_current_color = '{}';", color);
    runtime.execute_script("<set_color>", color_script)?;

    // 执行插件脚本
    runtime.execute_script(path_str, js_code)?;
}

// 命令执行时保留颜色
const cmdData = globalThis._rift_command_handlers['command'];
globalThis._rift_current_color = cmdData.color;  // 设置命令的颜色
handler(ctx);  // 执行命令
globalThis._rift_current_color = originalColor;  // 恢复原颜色
```

### 测试验证

#### 测试场景
创建了 `multi-version-test` 示例，包含两个版本的测试插件：
- `rift.test.v1@1.0.0`
- `rift.test.v2@2.0.0`

#### 测试结果

| 测试 | 命令 | 预期结果 | 实际结果 |
|------|------|---------|---------|
| 默认隔离 | `test-v1` | 只触发 v1 handlers | ✅ 只有 `[rift.test.v1] Handler v1.0.0` |
| 默认隔离 | `test-v2` | 只触发 v2 handlers | ✅ 只有 `[rift.test.v2] Handler v2.0.0` |
| 跨 scope | `test-all` (emit with `{scope: "all"}`) | 触发所有 handlers | ✅ v1 和 v2 都触发 |
| 精确 targeting | `test-target-v2` (emit with `{color: "..."}`) | 只触发 v2 | ✅ 只有 `[rift.test.v2] Handler v2.0.0` |

### 关键设计决策

1. **默认隔离** - `emit()` 默认只执行当前颜色的 handlers
2. **显式跨 scope** - 通过 `options` 参数显式指定目标 scope
3. **特殊值 "all"** - `scope: "all"` 跳过 scope 过滤，执行所有 handlers
4. **颜色字符串格式** - `plugin@version:scope` 简单可读，支持通配符

### 代码位置

- `src/vm/mod.rs` - JavaScript API 实现（on/emit/defineCommand）
- `src/main.rs` - 命令执行颜色保留
- `src/workspace/executor.rs` - ScriptContext 颜色设置
- `examples/multi-version-test/` - 测试示例

### 示例输出

```
Loading plugin: rift.test.v1
  Color: rift.test.v1@1.0.0:rift.test.v1
  - Command registered: test-v1
  - Event subscription: onTest (color: rift.test.v1@1.0.0:rift.test.v1)

Loading plugin: rift.test.v2
  Color: rift.test.v2@2.0.0:rift.test.v2
  - Command registered: test-v2
  - Event subscription: onTest (color: rift.test.v2@2.0.0:rift.test.v2)

$ rift test-v1
Executing command handler (color: rift.test.v1@1.0.0:rift.test.v1)
[rift.test.v1] Handler v1.0.0 executed!

$ rift test-all
[rift.test.v2] Handler v2.0.0 executed!
[rift.test.v1] Handler v1.0.0 executed!
```

### 下一步

1. **配置文件执行阶段** - metadata → dependencies → plugins 的顺序保证
2. **跨 scope 数据可见** - package 如何读取 workspace 的 config
3. **VM Scope 架构** - 完整的 ScopeManager 实现

---

## 跨 Scope Task 依赖实现

### 实现日期
2026-02-22

### 参考实现
基于 **Cake (C# Make)** 的 `IsDependentOn` 模式：
```csharp
Task("build")
    .IsDependentOn("clean")
    .IsDependentOn("restore");
```

### 核心实现

#### 1. Task 结构扩展

**新增字段：**
```rust
pub struct Task {
    // ... 原有字段
    /// Color context for this task (plugin@version:scope)
    pub color: Option<String>,
    /// Scope (package name) where this task is defined
    pub scope: String,
}
```

#### 2. 跨 Scope 语法解析

**支持格式：**
```typescript
// Local scope (本 package 的 task)
dependsOn: ["build"]

// Cross-scope (其他 package 的 task)
dependsOn: ["packageA:build"]
```

**解析函数：**
```rust
fn parse_task_reference(reference: &str) -> (Option<String>, String) {
    if let Some((scope, task_name)) = reference.split_once(':') {
        (Some(scope.to_string()), task_name.to_string())
    } else {
        (None, reference.to_string())
    }
}
```

#### 3. 跨 Scope Task 查找

```rust
pub fn find_task_in_scope(&self, task_name: &str, scope: Option<&str>) -> Option<Task> {
    let tasks = self.tasks.read().ok()?;
    if let Some(target_scope) = scope {
        // 在指定 scope 中查找
        tasks.values().find(|t| t.name == task_name && t.scope == target_scope).cloned()
    } else {
        // 查找所有 scope，返回第一个匹配
        tasks.values().find(|t| t.name == task_name).cloned()
    }
}
```

#### 4. 执行顺序解析（支持跨 Scope）

```rust
pub fn get_execution_order(&self, task_name: &str) -> Result<Vec<Task>> {
    // ... DFS 遍历依赖

    for dep in &task.dependencies {
        // 解析 "packageA:build" 格式
        let (scope, dep_task_name) = TaskManager::parse_task_reference(dep);

        // 如果指定了 scope，在该 scope 中查找 task
        let resolved_task_name = if let Some(target_scope) = scope {
            let found = tasks.values().find(|t| t.name == dep_task_name && t.scope == target_scope);
            if let Some(found_task) = found {
                found_task.name.clone()
            } else {
                return Err(anyhow!(
                    "Cross-scope dependency '{}' not found in scope '{}'",
                    dep_task_name,
                    target_scope
                ));
            }
        } else {
            dep_task_name.clone()
        };

        collect_deps(&resolved_task_name, tasks, to_execute, visited, visiting)?;
    }
    // ...
}
```

### 使用示例

#### 场景：packageB 依赖 packageA 的 build task

```typescript
// packageA/tasks.ts
tasks.register("build", {
    description: "Build package A",
    run: (ctx) => {
        console.log("Building package A...");
    }
});

// packageB/tasks.ts
tasks.register("test", {
    description: "Test package B",
    dependsOn: ["packageA:build"],  // 跨 scope 依赖
    run: (ctx) => {
        console.log("Testing package B...");
    }
});
```

#### 执行结果

```
$ rift packageB test
Executing 2 task(s) in order:
  - build (from packageA)
  - test (from packageB)

Building package A...
Testing package B...
```

### 测试用例

```rust
#[test]
fn test_cross_scope_task_dependencies() {
    let manager = TaskManager::new();

    // Register tasks in different scopes
    let package_a_build = Task::new("build".to_string())
        .with_scope("packageA".to_string())
        .with_color("unknown@unknown:packageA".to_string());

    let package_b_test = Task::new("test".to_string())
        .with_scope("packageB".to_string())
        .with_color("unknown@unknown:packageB".to_string())
        .with_dependency("packageA:build".to_string());

    manager.register_task("build".to_string(), package_a_build).unwrap();
    manager.register_task("test".to_string(), package_b_test).unwrap();

    // Get execution order
    let order = manager.get_execution_order("test").unwrap();

    assert_eq!(order.len(), 2);
    assert_eq!(order[0].name, "build");
    assert_eq!(order[0].scope, "packageA");
    assert_eq!(order[1].name, "test");
    assert_eq!(order[1].scope, "packageB");
}

#[test]
fn test_cross_scope_dependency_not_found() {
    let manager = TaskManager::new();

    let task = Task::new("test".to_string())
        .with_dependency("nonexistent:build".to_string());

    manager.register_task("test".to_string(), task).unwrap();

    // Should fail because the cross-scope dependency doesn't exist
    let result = manager.get_execution_order("test");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found in scope"));
}
```

### 代码位置

- `src/tasks/mod.rs` - Task 结构、TaskManager 跨 scope 解析
- `src/workspace/executor.rs` - Task 注册时带上 color 和 scope

### 设计决策

1. **简单语法** - `package:task` 格式直观易读
2. **明确错误** - 找不到跨 scope task 时返回清晰错误
3. **拓扑排序** - 自动解析跨 scope 依赖的执行顺序
4. **向后兼容** - 不带 scope 的 task name 保持原有行为

### 下一步

- [ ] 配置文件执行阶段保证（metadata → dependencies → plugins）
- [ ] ScopeBridge 数据传递机制（config API）

---

## 设计决策：不支持 npm 风格的包导入

### 背景
有建议让 Rift 支持 npm 风格的裸包导入，例如：
```typescript
import { utils } from 'my-utils-package';  // 裸包导入
```

### 决策：**暂不支持**

**Rift 目前专注于构建协调，不实现完整的包管理器功能。如果未来用户需求强烈，可以考虑支持 pnpm/npm 包解析。**

### 理由

1. **职责清晰**
   - Rift：构建协调（task 执行、依赖解析、插件隔离）
   - npm/deno/pnpm：应用代码的包管理

2. **避免复杂性**
   - 包版本冲突解析
   - Registry 集成
   - 依赖锁定
   - 缓存策略
   - 等等...

3. **当前设计已足够**
   ```typescript
   // ✅ 支持：相对路径导入（同一包内）
   import { helper } from './helper.ts';

   // ✅ 支持：rift: 内置 API
   import { addDependency } from "rift:api";

   // ❌ 不支持：裸包导入（应由 npm/deno/pnpm 处理）
   import { lodash } from 'lodash';
   ```

### 代码位置
`src/vm/mod.rs` 中的 `resolve_import_specifier` 函数：
```rust
if !(specifier.starts_with("./") || specifier.starts_with("../") || specifier.starts_with('/'))
{
    return Err(anyhow!(
        "Package manager / registry imports are forbidden: {}",
        specifier
    ));
}
```

### 如果需要共享实用函数？

**方案 1：放在同一个包内**
```
my-package/
  ├── helper.ts      # 实用函数
  ├── tasks.ts       # import { helper } from './helper.ts'
  └── Rift.toml
```

**方案 2：用真正的包管理器处理应用代码**
```
app/
  ├── node_modules/  # 由 npm/pnpm/yarn 管理
  ├── package.json
  └── build/
      └── Rift.toml  # Rift 只管理构建脚本
```

### 跨包数据共享的正确方式

使用 **Task 依赖** + **Config API**：
```typescript
// workspace/metadata.ts
config.defineVariable("organization", "MyOrg");

// packageA/dependencies.ts
const org = config.getVariable("organization");

// packageB/tasks.ts
tasks.register("build", {
    dependsOn: ["packageA:build"],  // Task 依赖
    run: (ctx) => { ... }
});
```