# Rift Plugin Registry Implementation

## Overview

This document summarizes the implementation of the plugin registry and git dependency support for Rift, following Cargo's registry design pattern.

## Design Decisions

### Registry Model: Cargo-Style

After analyzing NuGet, Cargo, Maven, and Gradle, we chose **Cargo's approach** for these reasons:

1. **Git-based index** - Decentralized, can be forked/mirrored
2. **Simple JSON Lines format** - Each line is a JSON object, efficient for incremental parsing
3. **Tiered directory structure** - Scales well with many packages
4. **Native git dependency support** - No need to publish for development/testing

### Index Structure

```
rift-index/
├── config.json                    # Download template
├── ri/ft/rift                     # Packages: ri/ft/rift.go
├── go/co/go-complex               # Packages: go/co/go-complex
└── 1/a                            # 1-char package names
```

**config.json:**
```json
{
  "dl": "https://cdn.rift-lang.org/plugins/{plugin}/{version}/download"
}
```

**Package file (e.g., ri/ft/rift.go):**
```json
{"name":"rift.go","vers":"1.0.0","url":"...","cksum":"sha256:..."}
{"name":"rift.go","vers":"1.1.0","url":"...","cksum":"sha256:..."}
```

## API Changes

### TypeScript API

**New Git Dependency Support:**
```typescript
// Registry version (from index)
addDependency({ name: "rift.go", version: "1.0.0" });

// Git direct reference
addDependency({
  name: "rift.go",
  source: "git",
  git: { url: "https://github.com/user/rift-go", ref: "main" }
});

// Configure index sources
addPluginIndex("https://github.com/rift-lang/index");
addPluginIndex("https://mirror.company.com/rift-index");
```

### Rust API Changes

**New Types:**
```rust
/// Git source information
pub struct GitSource {
    pub url: String,
    pub ref_: Option<String>,  // branch, tag, or commit
}

/// Extended dependency source
pub enum DependencySource {
    Explicit,   // From registry index
    Workspace,  // Local workspace package
    Inherit,    // From parent folder
    Git,        // Direct git reference (NEW)
}

/// Extended package reference
pub struct PackageReference {
    pub name: String,
    pub version: Option<String>,
    pub source: DependencySource,
    pub git: Option<GitSource>,  // NEW
    pub attributes: HashMap<String, serde_json::Value>,
}
```

## Files Modified

### 1. `src/vm/rift_api.d.ts`
- Added `GitSource` interface
- Extended `PackageReference` with `git` field and `source: "git"` option
- Added `addPluginIndex()` function declaration

### 2. `src/vm/mod.rs`
- Updated `RiftOpState` to include `plugin_indexes: Vec<String>`
- Updated `RawScriptResult` to include `plugin_indexes`
- Added `op_rift_add_plugin_index` op function
- Updated `addDependency` JS runtime to handle git source

### 3. `src/workspace/executor.rs`
- Added `GitSource` struct
- Extended `DependencySource` enum with `Git` variant
- Extended `PackageReference` with `git` field
- Added `with_git()` constructor
- Updated `from_dependency_value()` to extract git info from attributes
- Updated `resolve_dependencies()` to handle git dependencies

## Usage Examples

### User Configuration

```typescript
// In workspace dependencies.ts
addPluginIndex("https://github.com/rift-lang/index");

// Use registry plugin
addPlugin({ name: "rift.go", version: "1.0.0" });

// Or use git directly for development
addPlugin({
  name: "rift.go",
  source: "git",
  git: { url: "https://github.com/user/rift-go", ref: "dev" }
});
```

### Dependency Resolution Flow

```
addDependency({ name: "pkg", source: "git", git: {...} })
       ↓
Stored in PackageReference with source=Git
       ↓
resolve_dependencies() keeps it as-is
       ↓
Plugin system handles git clone/checkout
```

## Future Work

### Not Yet Implemented

1. **Index Query Logic** - The actual code to:
   - Fetch index from configured URLs
   - Parse JSON Lines format
   - Resolve package names to download URLs

2. **Download/Cache System** - Code to:
   - Download plugins from registry
   - Cache locally
   - Verify checksums

3. **Git Clone Logic** - Code to:
   - Clone git repositories
   - Checkout specific refs
   - Cache cloned repos

4. **Path Dependencies** - (Discussed but not implemented)
   ```typescript
   addDependency({ name: "local-pkg", source: "path", path: "../local-pkg" });
   ```
   This exists in Cargo but Go's `go.work` handles this for Go projects.

### Implementation Notes

- The current implementation provides the **data structures and API surface**
- Actual index fetching and plugin loading is handled by the plugin system
- This design allows the registry to be language-agnostic

## Testing

To test the implementation:

```bash
cargo build
cargo test
```

Example TypeScript usage:
```typescript
// dependencies.ts
addPluginIndex("https://github.com/rift-lang/index");
addDependency({ name: "rift.go", version: "1.0.0" });
addDependency({ name: "dev-plugin", source: "git", git: { url: "https://..." } });
```

## References

- [Cargo Registry Protocol](https://doc.rust-lang.org/cargo/reference/registries.html)
- [NuGet Service Index](https://learn.microsoft.com/en-us/nuget/api/service-index)
- [Maven Metadata](https://maven.apache.org/repositories/metadata.html)
