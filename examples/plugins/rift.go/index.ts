// rift.go plugin
// Provides Go project support
// Subscribes to the "onGenerate" event to generate go.mod files
//
// Note: The Rift API is available globally without imports
// - defineCommand(name, handler)
// - emit(eventName, context)
// - on(eventName, handler)
// - writeFile(path, content)

// Export Go API for other packages to use in their configure scripts
(globalThis as any).go = {
    setVersion: (version: string) => {
        // Store the version for this package
        // The config system will capture this
        const config = (globalThis as any).config;
        if (config?.set) {
            config.set("go.version", version);
        }
    }
};

on("onGenerate", (ctx) => {
    console.log(`[rift.go] Generating go.mod files...`);

    // Helper to normalize Windows paths (remove \\?\ prefix)
    const normalizePath = (p: string): string => {
        // Remove Windows extended path prefix \\?\
        return p.replace(/\\\\\?\\/g, '');
    };

    // Get configuration from context
    const config = ctx.config || {};

    // Iterate through all packages in the workspace
    if (ctx.packages) {
        for (const [name, path] of ctx.packages) {
            // Only generate for actual project packages (not plugins, not workspace)
            if (name === 'plugin-test-workspace' || name === 'go-workspace' || name.startsWith('rift.')) {
                continue; // Skip workspace and plugins
            }

            // Get Go version from package config, default to 1.21
            const pkgConfig = config[name] || {};
            const goVersion = pkgConfig['go.version'] || '1.21';

            // Normalize path to handle Windows extended path prefix
            const normalizedPath = normalizePath(path);

            // Generate go.mod content
            const goModContent = `module ${name}

go ${goVersion}

// Rift generated this file
`;

            // Use normalized path for file writing
            const goModPath = `${normalizedPath}/go.mod`;

            if (writeFile(goModPath, goModContent)) {
                // File generated successfully
            } else {
                console.error(`[rift.go] Failed to generate: ${goModPath}`);
            }
        }
    }

    console.log(`[rift.go] go.mod generation complete.`);
});
