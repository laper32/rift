# Rift Implementation Status

**Branch:** `feat/re-impl` (re-implementation from C#-style to TypeScript-style API)

## Completed Features

### ✅ 1. Core Scanning System
**Location:** `src/workspace/`

- Workspace and package scanning from `Rift.toml` manifests
- Support for all package types: Workspace, Folder, Project, Target, Plugin
- Parent-child relationship tracking
- Manifest path resolution

**Key files:**
- `src/workspace/mod.rs` - Main scanner
- `src/workspace/package.rs` - Package types (MaybePackage enum)
- `src/workspace/graph.rs` - Dependency graph for script imports

### ✅ 2. TypeScript Script Execution
**Location:** `src/vm/mod.rs`

- Deno-based VM for executing `.ts` configuration scripts
- Script dependency tracking (imports, requires)
- Linting with deno-lint
- Module graph building

**Execution order:**
1. Workspace scripts
2. Folder scripts
3. Plugin scripts (first, to load APIs)
4. Project/Target scripts

### ✅ 3. TypeScript-Style Functional API
**Location:** `src/vm/mod.rs`, `src/vm/rift_api.d.ts`

**Available functions:**
```typescript
// Dependencies
addDependency({ name, version?, source?, git?, path?, attributes? })
addDependency(dependencies[])
excludeDependency(name)

// Plugins
addPlugin({ name, version })
addPlugin(plugins[])

// Package configuration
configurePackage(config => {
  config.set(key, value)
  value = config.get(key)
})

// Tasks
defineTask({
  name,
  description,
  dependencies[],
  isCommand,
  action
})
```

**Note:** Plugin indexes are configured in `.rift/config.toml`, not via TypeScript API.

**Context API:**
```typescript
Rift.package     // Current package info
Rift.parent      // Parent package (if any)
Rift.root        // Workspace root
Rift.find(name)  // Find any package
```

### ✅ 4. Dependency Resolution
**Location:** `src/workspace/executor.rs`

**Dependency sources:**
- `explicit` - Version specified, looked up in registry (future)
- `workspace` - Reference to workspace dependencies
- `inherit` - Inherit from parent folder
- `git` - Direct git repository reference
- `path` - Local filesystem path (NEW)

**Resolution flow:**
1. Collect all dependencies from scripts
2. Resolve workspace references
3. Resolve inheritance from parents
4. Keep git/explicit references as-is

### ✅ 5. Git Dependency Support (NEW)
**Location:** `src/workspace/executor.rs`, `src/vm/mod.rs`

**Data structures:**
```rust
pub struct GitSource {
    pub url: String,
    pub ref_: Option<String>,  // branch, tag, or commit
}

pub enum DependencySource {
    Explicit,
    Workspace,
    Inherit,
    Git,  // NEW
}
```

**TypeScript API:**
```typescript
addDependency({
  name: "rift.go",
  source: "git",
  git: { url: "https://github.com/user/rift-go", ref: "main" }
});
```

**Status:** Data structures and API complete. Git clone logic NOT implemented.

### ✅ 6. Rift Configuration (NEW)
**Location:** `src/config/mod.rs`

**Configuration file:** `.rift/config.toml`

```toml
[index]
sources = [
    "https://github.com/rift-lang/index",
    "https://my-mirror.com/rift-index"
]
```

**Features:**
- Read from workspace directory (searches upward)
- Falls back to `~/.rift/config.toml`
- Default index: `https://github.com/rift-lang/index`

**Status:** Complete. Index fetching implemented.

### ✅ 7. Index/Registry System (NEW)

**Location:** `src/index/mod.rs`, `src/main.rs:query_explicit_dependencies()`

- HTTP index fetching with `ureq` crate
- JSON Lines parsing (Cargo-style)
- Cargo-style package file path resolution
- Local caching in `~/.rift/index/`
- Multi-index support (query multiple indexes)

**API:**
```rust
pub fn fetch_index(url: &str) -> Result<CachedIndex>
pub fn query_package(index_url: &str, package_name: &str, version: &str) -> Result<PackageEntry>
fn query_explicit_dependencies(root: &Path, script_results: &HashMap<String, ScriptResult>) -> Result<()>
pub fn package_file_path(name: &str) -> String
```

**Cargo-style package file path:**
```
packages/
├── 1-char/              # Single character names
├── 2-chars/             # Two character names
├── 3-chars/             # Three character names
└── ab/                  # 4+ character names
    └── cd/
        └── package-name
```

**Usage:**
```typescript
// In .rift/config.toml
[index]
sources = ["https://github.com/rift-lang/index"]

// In dependencies.ts
addDependency({ name: "lodash", version: "4.17.21" });
```

When running `rift generate`, explicit dependencies are queried from configured indexes.

**Status:** Query implemented. Download and checksum verification NOT implemented.

### ✅ 8. Task System
**Location:** `src/tasks/mod.rs`, `src/workspace/executor.rs`

- Task registration from scripts
- Task dependencies
- Command exposure (CLI integration)
- Package-specific working directory

**Data structure:**
```rust
pub struct Task {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub is_command: bool,
    pub action: Option<String>,
    pub package_name: String,
    pub package_path: Option<String>,
}
```

### ✅ 9. Go Workspace Generation
**Location:** `src/main.rs` (cmd_generate)

**Features:**
- Generate `go.work` file
- Generate `go.mod` files for each package
- Module naming: `workspace/package` format
- Dependency resolution for workspace packages

**Example:**
```bash
cd examples/go-complex
rift generate
# Creates go.work and go.mod files
```

**Note:** The `generate` command is currently builtin for development. In the future, this should be provided by the `rift.go` plugin as a registered task.

### ✅ 10. Plugin-Provided Commands (NEW)

**Architecture:** Commands are tasks registered by plugins, not hardcoded in CLI.

**How it works:**
1. Plugins register tasks with `isCommand: true`
2. Tasks are available as `rift <taskname>`
3. CLI first checks if argument is a subcommand (scan, tasks)
4. If not, treats it as a task name

**Example (Go plugin):**
```typescript
// In rift.go plugin
defineTask({
  name: "generate",
  description: "Generate Go workspace files",
  isCommand: true,
  action: () => {
    // Generate go.work, go.mod files
  }
});
```

**User experience:**
```bash
# These are equivalent:
rift generate        # Execute generate task (provided by rift.go plugin)
rift tasks           # List builtin subcommand
```

**Development fallback:**
- Currently, `generate` has a builtin implementation for development
- When no plugin provides the task, builtin is used with a deprecation notice
- Eventually, plugins will fully override builtin behavior

**Benefits:**
- Language plugins provide their own generate logic
- No core changes needed for new languages
- Consistent task/command interface

## Partially Implemented

### ⚠️ Plugin System
**Location:** `src/plugin/`

- Plugin manifest parsing exists
- Plugin can export `index.ts` as entry point
- Plugin loading during script execution works
- **BUT:** Actual plugin functionality export is limited

**What works:**
- Plugins can register commands
- Plugins can subscribe to events
- Plugins can register tasks

**What doesn't:**
- Plugin sandboxing
- Plugin lifecycle management (setup/teardown)
- Plugin API versioning

## Not Implemented

### ❌ Plugin Download

**Not implemented:**
- Download plugins from registry URLs
- Cache downloaded plugins locally
- Verify checksums

### ❌ Package Download

**Not implemented:**
- Download explicit dependencies from registry
- Cache downloaded packages locally
- Verify checksums

**Use cases:**
- Go's `replace` directive equivalent
- Local development with uncommitted changes
- Monorepo local package references
- Forking/testing local modifications

### ❌ Lock File

**No lock file format defined yet.**
- Similar to `Cargo.lock` or `package-lock.json`
- Would lock dependency versions

### ❌ Task Execution

**Tasks can be registered but:**
- Task actions (JS functions) can't be executed yet
- No `rift run <task>` command
- No dependency execution order

## File Manifest

### Core Types
```
src/
├── main.rs              # CLI entry, commands (scan, generate)
├── cli.rs               # CLI argument parsing (unused?)
├── config/              # Rift configuration (NEW)
│   └── mod.rs           # .rift/config.toml reader
├── git/                 # Git operations
│   └── mod.rs           # Clone, fetch, checkout, caching
├── index/               # Package index for registry (NEW)
│   └── mod.rs           # Index fetch, query, Cargo-style package file path
├── manifest/            # TOML parsing
│   ├── mod.rs
│   ├── real.rs          # Project, Target
│   ├── rift.rs          # Plugin
│   └── virtual.rs       # Workspace, Folder
├── schema/              # Schema types
├── workspace/           # Core logic
│   ├── mod.rs           # Scanner
│   ├── executor.rs      # Script execution, dependency resolution
│   ├── graph.rs         # Import dependency graph
│   └── package.rs       # MaybePackage enum
├── vm/                  # Deno VM
│   ├── mod.rs           # VM implementation, ops
│   └── rift_api.d.ts    # TypeScript type definitions
├── plugin/              # Plugin system (partial)
├── tasks/               # Task registration
└── vm/                  # (duplicate?) - check this
```

## Current Todo

If continuing development, priority order:

1. **Plugin download** - Download and cache plugins from registry
2. **Package download** - Download explicit dependencies from registry
3. **Checksum verification** - Verify downloaded packages
4. **Task execution** - Run tasks with dependency ordering
5. **Lock file** - Version locking

## Key Design Decisions

1. **TypeScript over C#** - Functional API: `addDependency()` not `Dependencies.add()`
2. **Cargo-style registry** - Git-based index, JSON Lines format
3. **Rift is a coordination layer** - Doesn't parse package formats, delegates to plugins
4. **Script-based configuration** - All config in `.ts` files, not TOML
5. **Plugin-provided commands** - Commands like `generate` are registered by plugins, not hardcoded
6. **Go workspace model** - `go.work`-style workspace for language integration

## Quick Reference

### Manifest Structure
```toml
# Workspace (root)
[workspace]
name = "my-workspace"
members = ["services/*", "libs/*"]
dependencies = "dependencies.ts"

# Folder (logical grouping)
[folder]
name = "services"
members = ["api", "frontend"]
dependencies = "dependencies.ts"

# Target (buildable)
[target]
name = "api"
type = "bin"  # | "lib" | "obj"
dependencies = "dependencies.ts"
plugins = "plugins.ts"
tasks = "tasks.ts"
```

### Dependency Examples
```typescript
// Explicit (registry)
addDependency({ name: "lodash", version: "4.17.21" });

// Workspace
addDependency({ name: "shared-utils", source: "workspace" });

// Inherit
addDependency({ name: "config", source: "inherit" });

// Git
addDependency({
  name: "rift.go",
  source: "git",
  git: { url: "https://github.com/user/rift-go", ref: "main" }
});

// Path (NEW)
addDependency({
  name: "local-pkg",
  source: "path",
  path: { path: "../local-pkg" }
});
// or shorthand
addDependency({
  name: "local-pkg",
  source: "path",
  path: "../local-pkg"
});
```

## Dependencies

```toml
[dependencies]
deno_ast = { version = "0.53.0", features = ["transpiling", "visit"] }
deno_lint = "0.83.0"
deno_core = "0.385.0"
deno_error = "0.6.0"
anyhow = "1.0.102"
tokio = { version = "1.49.0", features = ["full"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0"
toml = "0.9.8"
clap = { version = "4.5.60", features = ["derive"] }
glob = "0.3"
git2 = "0.19"      # Git operations
dirs = "5.0"       # Home directory detection
ureq = { version = "2.12", features = ["json"] }  # HTTP client for index fetching
dirs = "5.0"       # NEW - Home directory detection
```

## Testing

```bash
# Build
cargo build

# Scan workspace
rift scan

# Generate Go files (example)
cd examples/go-complex
rift generate

# Run tests
cargo test
```
