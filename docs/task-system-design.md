# Task System Design

## Overview

This document analyzes the task system design in Rift, focusing on how plugins can define reusable task sets that projects can use directly.

## Current Implementation

### Status: ✅ Working

The current implementation successfully executes tasks with dependencies:

- **Single task execution**: `rift build` works
- **Dependency resolution**: `rift test` correctly executes `build` first, then `test`
- **Complex dependencies**: `rift ci` correctly executes `clean → build → test → ci`

### How It Works

```typescript
// In project tasks.ts
tasks.register("build", (task) => {
    task.description = "Build the application";
    task.do(() => {
        console.log("[myapp] Building application...");
    });
});

tasks.register("test", (task) => {
    task.description = "Run tests";
    task.dependsOn("build");
    task.do(() => {
        console.log("[myapp] Running tests...");
    });
});
```

**Execution flow:**
```
User runs: rift test
    ↓
1. Load workspace and discover all tasks
2. Resolve dependencies (topological sort)
3. Execute each task:
   - Load tasks.ts
   - Create JS runtime
   - Execute task action
    ↓
Output:
  [myapp] Building application...
  [myapp] Running tests...
```

## Current Limitations

### Problem: No Reusable Task Definitions

Plugins cannot define task sets that projects can reuse. Each project must define all tasks in its own `tasks.ts`.

**Example of what we want but can't do:**
```typescript
// In a plugin (e.g., rust plugin)
// Define tasks once...
const rustTasks = {
    build: { ... },
    test: { ... }
};

// ...but how does a project use them?
```

### Root Cause: VM Isolation

Each task execution creates a **new isolated JS runtime**:
- Task actions are stored in `globalThis._rift_task_actions` during script execution
- But this state is **not shared** between separate `execute_script()` calls
- Therefore, plugins cannot pre-register tasks for projects to use

## Case Study: `generate` Command

### How Generate Works

```
User runs: rift generate
    ↓
1. Load all plugins (first time)
   - rift.generate: defineCommand("generate", handler)
   - rift.go: on("onGenerate", handler)
    ↓
2. Execute command
   - Reload all plugins (second time)
   - Call generate handler
   - emit("onGenerate", {...})
    ↓
3. Event subscribers respond
   - rift.go handler runs
   - Generates go.mod for each package
```

### Key Observations

1. **Event-based coordination**: `emit()` → `on()` pattern
2. **Double loading**: Plugins loaded twice (discovery + execution)
3. **Simple commands**: No parameters, no dependencies

### Can We Use This Pattern for Tasks?

**Differences:**
| Feature | `generate` command | Tasks |
|---------|-------------------|-------|
| Definition | In plugin | In project's tasks.ts |
| Parameters | None | May need customization |
| Dependencies | None | Yes, tasks can depend on each other |
| Context | Workspace | Package-specific |

## Design Problem

### The Core Question

How should a project `tasks.ts` reference/use tasks defined by a plugin?

**Scenario:** A `rust` plugin defines `build` and `test` tasks. How does a Rust project use them?

```typescript
// In project tasks.ts
// What should the syntax be?

// Option A: Direct reference?
tasks.use("rust:build");

// Option B: Configuration?
tasks.use("rust", ["build", "test"]);

// Option C: Template application?
tasks.apply("rust:build", { name: "build" });
```

### Requirements

1. **Reusability**: Plugins define tasks once, projects use them
2. **Customization**: Projects should be able to customize task behavior
3. **Namespacing**: Avoid conflicts between plugins
4. **Simplicity**: Easy to understand and use
5. **Compatibility**: Work with existing dependency resolution

## Proposed Solutions

### Solution 1: Task Templates

```typescript
// Plugin: rust.plugin
tasks.defineTemplate("rust:cargoBuild", {
    description: "Build with Cargo",
    run: (ctx) => exec("cargo", ["build"])
});

// Project: my-rust-app/tasks.ts
tasks.apply("rust:cargoBuild", {
    name: "build",
    description: "Build my Rust app"
});
```

**Pros:**
- Clear separation of concerns
- Highly customizable
- Easy to understand

**Cons:**
- Complex implementation
- Potential conflicts
- Hard to debug

### Solution 2: Global Registration

```typescript
// Plugin: rust.plugin
tasks.register("cargoBuild", {
    description: "Build with Cargo",
    run: (ctx) => exec("cargo", ["build"])
});

// Project: my-rust-app/tasks.ts
// Task already available, just create alias
tasks.register("build", {
    dependsOn: ["cargoBuild"]
});
```

**Pros:**
- Simple implementation
- Works like npm scripts

**Cons:**
- Naming conflicts
- Poor customization
- Load order sensitivity

### Solution 3: Mixin/Compose

```typescript
// Plugin: rust.plugin
export const taskSets = {
    cargo: {
        build: { run: (ctx) => exec("cargo", ["build"]) },
        test: { run: (ctx) => exec("cargo", ["test"]) }
    }
};

// Project: my-rust-app/tasks.ts
import { taskSets } from "rust.plugin";
tasks.mixin(taskSets.cargo);
```

**Pros:**
- Most flexible
- Composition over inheritance
- Version isolation

**Cons:**
- Complex syntax
- Steeper learning curve
- Complex implementation

### Solution 4: Hybrid (Template + Namespace)

```typescript
// Plugin: rust.plugin
tasks.defineTemplate("rust:cargoBuild", { ... });

// Project: my-rust-app/tasks.ts
tasks.apply("rust:cargoBuild", { name: "build" });
```

**Pros:**
- Combines flexibility of templates with namespacing
- Clear naming convention
- Good balance

**Cons:**
- Still relatively complex

## Open Questions

1. **Syntax**: What should the API look like for projects to use plugin tasks?
2. **VM Persistence**: Should plugins maintain state across executions?
3. **Task Location**: Should plugin tasks be defined in:
   - Plugin's `index.ts`?
   - A separate `tasks.ts` in the plugin?
   - A special `tasks.d.ts` or similar?

4. **Customization**: How much should projects be able to customize plugin tasks?
   - Just names/aliases?
   - Parameters?
   - Full override of run function?

## Next Steps

1. **Decide on API** based on usage scenarios
2. **Design VM architecture** to support shared task definitions
3. **Implement** the chosen approach
4. **Test** with real-world examples (Rust, Go, TypeScript plugins)
