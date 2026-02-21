/**
 * Rift Core API Types
 * This is the built-in API available to all Rift scripts
 */

/**
 * Package reference - represents a dependency or plugin
 *
 * Rift is a coordination layer - it does NOT parse package formats.
 * Each language plugin (rift.go, rift.ts, etc.) handles its own format.
 *
 * The `source` field determines where the dependency comes from:
 * - "explicit" (default): version is explicitly specified in `version` field
 * - "workspace": reference a package defined in the workspace
 * - "inherit": inherit from parent/folder definitions
 *
 * Examples:
 *   // Explicit version
 *   addDependency({ name: "lodash", version: "4.17.21" });
 *
 *   // Workspace reference
 *   addDependency({ name: "shared-utils", source: "workspace" });
 *
 *   // Inherit from parent
 *   addDependency({ name: "config", source: "inherit" });
 *
 *   // With attributes
 *   addDependency({ name: "react", version: "18.0.0", attributes: { dev: true } });
 */
export interface PackageReference {
    /** Package name in the format expected by the target plugin */
    name: string;
    /** Optional version constraint (used when source is "explicit") */
    version?: string;
    /** Dependency source: "explicit" (default), "workspace", or "inherit" */
    source?: "explicit" | "workspace" | "inherit";
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
