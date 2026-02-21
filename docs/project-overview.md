# Rift - Polyglot Build System

## Overview

Rift is a **coordination layer** for polyglot projects. It does NOT parse package formats itself - instead, it delegates to language-specific plugins (rift.go, rift.ts, etc.) that handle their native formats.

### Key Design Principle

> **Rift is a coordination layer, not a package manager.**
>
> Each language plugin handles its own format. Rift provides:
> - Workspace scanning and dependency resolution
> - Script-based configuration (TypeScript)
> - Task execution and CLI generation
> - Plugin system for extensibility

### Why Rift Exists

Originally created to solve problems with:
- **Go + Protobuf microservices** - protoc Docker issues, cross-platform builds
- **C++/C# game mod projects** - Windows (Visual Studio) vs Linux (CMake) build differences
- **Vendor dependency hell** - Protobuf 3.21.8 vendoring problems

## Project Structure

```
rift/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs               # Command-line interface
│   ├── manifest/            # TOML manifest parsing
│   │   ├── mod.rs           # Main types
│   │   ├── real.rs          # Project/Target manifests
│   │   ├── rift.rs          # Plugin manifests
│   │   └── virtual.rs       # Workspace/Folder manifests
│   ├── schema/              # Schema definitions
│   ├── workspace/           # Workspace scanning & resolution
│   │   ├── mod.rs
│   │   ├── executor.rs      # Script execution
│   │   ├── graph.rs         # Dependency graph
│   │   └── package.rs       # Package types
│   ├── vm/                  # Deno VM for TypeScript execution
│   │   ├── mod.rs           # VM and ops
│   │   └── rift_api.d.ts    # TypeScript API definitions
│   ├── plugin/              # Plugin system
│   ├── tasks/               # Task management
│   └── vm/                  # Virtual machine (Deno)
├── examples/                # Example projects
│   ├── go-complex/          # Complex Go workspace example
│   └── ...
└── docs/                    # Documentation
```

## Current Branch: `feat/re-impl`

This is a **re-implementation** of Rift with:
- TypeScript-based configuration (not C# namespaces)
- Improved workspace scanning
- Better error handling
- Plugin registry support (in progress)

## Configuration Model

### Manifest Files (Rift.toml)

**Workspace (root):**
```toml
[workspace]
name = "my-workspace"
members = ["services/*", "libs/*"]
dependencies = "dependencies.ts"    # Workspace-level deps
```

**Folder:**
```toml
[folder]
name = "services"
members = ["api", "frontend"]
dependencies = "dependencies.ts"    # Folder-level deps (inherited)
```

**Project/Target:**
```toml
[target]
name = "api"
dependencies = "dependencies.ts"
plugins = "plugins.ts"
tasks = "tasks.ts"
```

### Script Configuration (TypeScript)

**dependencies.ts:**
```typescript
// Explicit version (from registry)
addDependency({ name: "rift.go", version: "1.0.0" });

// Workspace reference
addDependency({ name: "shared-utils", source: "workspace" });

// Inherit from parent
addDependency({ name: "config", source: "inherit" });

// Git direct reference
addDependency({
  name: "rift.go",
  source: "git",
  git: { url: "https://github.com/user/rift-go", ref: "main" }
});

// Configure index sources
addPluginIndex("https://github.com/rift-lang/index");
```

**plugins.ts:**
```typescript
addPlugin({ name: "rift.go", version: "1.0.0" });
addPlugin({ name: "rift.ts", version: "2.0.0" });
```

**tasks.ts:**
```typescript
defineTask({
  name: "build",
  description: "Build the project",
  dependencies: ["clean"],
  isCommand: true,
  action: () => {
    console.log("Building...");
  }
});
```

## Core Concepts

### Dependency Sources

| Source | Description | Example |
|--------|-------------|---------|
| `explicit` | Version from registry | `{ name: "lodash", version: "4.17.21" }` |
| `workspace` | From workspace deps | `{ name: "pkg", source: "workspace" }` |
| `inherit` | From parent folder | `{ name: "pkg", source: "inherit" }` |
| `git` | Direct git reference | `{ name: "pkg", source: "git", git: {...} }` |

### Package Types

| Type | Manifest Key | Description |
|------|--------------|-------------|
| **Workspace** | `[workspace]` | Root workspace definition |
| **Folder** | `[folder]` | Logical folder grouping |
| **Project** | `[project]` | Package with metadata (authors, version, can have members) |
| **Target** | `[target]` | Build target: `type = "bin" \| "lib" \| "obj"` |
| **Plugin** | `[plugin]` | Rift plugin package |

**Target Types:**
- `bin` - Executable binary
- `lib` - Library
- `obj` - Object file / intermediate artifact

### Rift Context API

Available in all scripts:
```typescript
interface RiftContext {
  readonly package: PackageInfo;      // Current package
  readonly parent: PackageInfo;       // Parent (if any)
  readonly root: PackageInfo;         // Workspace root
  find(name: string): PackageInfo;    // Find any package
}

// Usage
console.log(Rift.package.name);        // Current package
console.log(Rift.root.path);           // Workspace path
const common = Rift.find("common");    // Find workspace member
```

## Commands

### Scanning
```bash
rift scan                    # Scan and display workspace
rift scan /path/to/project   # Scan specific path
```

### Generation
```bash
rift generate                # Generate project files
# Generates go.work, go.mod, etc. based on resolved dependencies
```

### Running Tasks
```bash
rift run <task>              # Run a task
rift build                   # Run 'build' task if marked as command
```

## Plugin System (In Progress)

### Plugin Registry Design

Following **Cargo's registry model**:
- Git-based index (decentralized)
- JSON Lines format (efficient parsing)
- Tiered directory structure (scales well)

### Index Structure
```
rift-index/
├── config.json                    # Download template
├── ri/ft/rift                     # ri/ft/rift.go
└── go/co/go-complex               # go/co/go-complex
```

**config.json:**
```json
{
  "dl": "https://cdn.rift-lang.org/plugins/{plugin}/{version}/download"
}
```

**Package file:**
```json
{"name":"rift.go","vers":"1.0.0","url":"...","cksum":"sha256:..."}
{"name":"rift.go","vers":"1.1.0","url":"...","cksum":"sha256:..."}
```

### Plugin API

Plugins export commands via `index.ts`:
```typescript
// plugins/rift-plugin-example/index.ts
export function setup() {
  console.log("Plugin initialized!");
}

export function teardown() {
  console.log("Plugin cleanup!");
}
```

## Development Guide

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build
```

### Testing
```bash
cargo test               # Run all tests
cargo test --workspace   # Workspace tests only
```

### Running Example
```bash
cd examples/go-complex
rift generate            # Generate go.work and go.mod files
go build ./...           # Build with Go
```

### Adding a New Command

1. Define in `src/cli.rs`:
```rust
fn cmd_my_command(args: CliArgs) -> Result<()> {
    // Implementation
}
```

2. Register in `main()`:
```rust
Command::new("my-command").about("My command").handler(|args| {
    cmd_my_command(args)
}),
```

### Adding a New Op (VM Function)

1. Define op function in `src/vm/mod.rs`:
```rust
#[op2(fast)]
fn op_rift_my_function(
    state: &mut OpState,
    #[string] param: String,
) {
    // Implementation
}
```

2. Register in `rift_runtime` extension:
```rust
ops = [
    op_rift_my_function,
    // ...
],
```

3. Add JavaScript wrapper in `RIFT_API_JS`:
```javascript
function myFunction(param) {
    ops.op_rift_my_function(param);
}
```

## Example: Go Workspace Generation

The `rift generate` command demonstrates the full workflow:

1. **Scan workspace** - Find all packages and manifests
2. **Execute scripts** - Run dependencies.ts, plugins.ts, etc.
3. **Resolve dependencies** - Handle workspace/inherit/git sources
4. **Generate files** - Create go.work, go.mod based on results

See `examples/go-complex/` for a complete working example.

## Testing the Implementation

```bash
# Navigate to example
cd examples/go-complex

# Generate Go files
rift generate

# Verify generated files
cat go.work
cat services/api/go.mod

# Build with Go
go build ./...

# Run the API
cd services/api
go run .
```

## Future Work

### Planned Features
- [ ] Plugin download and caching
- [ ] Git dependency cloning
- [ ] Path-based dependencies
- [ ] Plugin index fetching
- [ ] Command execution with task actions
- [ ] Configuration file generation for other languages

### Under Discussion
- [ ] Lock file format (like Cargo.lock or package-lock.json)
- [ ] Private registry support
- [ ] Plugin signing and verification

## Resources

- [Cargo Registries](https://doc.rust-lang.org/cargo/reference/registries.html) - Registry design reference
- [Go Workspaces](https://go.dev/ref/mod#workspaces) - Multi-module workspace reference
- [TypeScript API](src/vm/rift_api.d.ts) - Full API definitions

## Contributing

1. All configuration is TypeScript-based (not C# namespaces)
2. Dependencies use functional API: `addDependency()`, not `Dependencies.add()`
3. Source types: `explicit`, `workspace`, `inherit`, `git`
4. The VM uses Deno with custom ops for Rift API
5. Scripts are executed in order: workspace → folder → plugins → projects
