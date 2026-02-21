/**
 * Rift Core API Types
 * This is the built-in API available to all Rift scripts
 */

/**
 * Git dependency source
 */
export interface GitSource {
    /** Git repository URL */
    url: string;
    /** Branch, tag, or commit hash (optional, defaults to main/master) */
    ref?: string;
}

/**
 * Path dependency source
 */
export interface PathSource {
    /** Filesystem path (relative or absolute) */
    path: string;
}

/**
 * Package reference - represents a dependency or plugin
 *
 * Rift is a coordination layer - it does NOT parse package formats.
 * Each language plugin (rift.go, rift.ts, etc.) handles its own format.
 *
 * The `source` field determines where the dependency comes from:
 * - "explicit" (default): version is explicitly specified, looked up in registry index
 * - "workspace": reference a package defined in the workspace
 * - "inherit": inherit from parent/folder definitions
 * - "git": direct git repository reference
 * - "path": local filesystem path (like Go's replace directive)
 *
 * Examples:
 *   // Explicit version (from registry)
 *   addDependency({ name: "rift.go", version: "1.0.0" });
 *
 *   // Workspace reference
 *   addDependency({ name: "shared-utils", source: "workspace" });
 *
 *   // Inherit from parent
 *   addDependency({ name: "config", source: "inherit" });
 *
 *   // Git direct reference
 *   addDependency({ name: "rift.go", source: "git", git: { url: "https://github.com/user/rift-go", ref: "main" } });
 *
 *   // Path reference (local filesystem)
 *   addDependency({ name: "local-pkg", source: "path", path: "../local-pkg" });
 *
 *   // With attributes
 *   addDependency({ name: "react", version: "18.0.0", attributes: { dev: true } });
 */
export interface PackageReference {
    /** Package name in the format expected by the target plugin */
    name: string;
    /** Optional version constraint (used when source is "explicit") */
    version?: string;
    /** Dependency source: "explicit" (default), "workspace", "inherit", "git", or "path" */
    source?: "explicit" | "workspace" | "inherit" | "git" | "path";
    /** Git source configuration (required when source is "git") */
    git?: GitSource;
    /** Path source configuration (required when source is "path") */
    path?: PathSource;
    /** Optional attributes for language-specific metadata */
    attributes?: Record<string, any>;
}

/**
 * Add a single dependency or multiple dependencies
 * @param dependency - The package reference(s) to add
 */
export declare function addDependency(dependency: PackageReference): void;
export declare function addDependency(dependencies: PackageReference[]): void;

/**
 * Exclude a dependency (prevents inheritance)
 * @param name - The package name to exclude
 */
export declare function excludeDependency(name: string): void;

/**
 * Add a single plugin or multiple plugins
 * @param plugin - The plugin reference(s) to register
 */
export declare function addPlugin(plugin: PackageReference): void;
export declare function addPlugin(plugins: PackageReference[]): void;

/**
 * Configure the current package
 * @param configure - Configuration callback
 */
export declare function configurePackage(
    configure: (config: PackageConfiguration) => void
): void;

/**
 * Package configuration API
 */
export interface PackageConfiguration {
    /**
     * Set a configuration value
     * @param key - Configuration key
     * @param value - Configuration value
     */
    set(key: string, value: string | number | boolean): void;

    /**
     * Get a configuration value
     * @param key - Configuration key
     * @returns The configuration value, or undefined if not set
     */
    get(key: string): string | number | boolean | undefined;
}

/**
 * Package path information
 */
export interface PackageInfo {
    /** Package name */
    name: string;
    /** Package manifest directory path */
    path: string;
}

/**
 * Rift built-in context API
 */
export interface RiftContext {
    /** Current package information */
    readonly package: PackageInfo;
    /** Parent package information (undefined if none) */
    readonly parent: PackageInfo | undefined;
    /** Root workspace information */
    readonly root: PackageInfo;
    /**
     * Find a package by name
     * @param name - Package name to find
     * @returns Package info, or undefined if not found
     */
    find(name: string): PackageInfo | undefined;
}

/**
 * Global Rift API
 */
declare global {
    /** Rift built-in context - provides path and package information */
    const Rift: RiftContext;
}
