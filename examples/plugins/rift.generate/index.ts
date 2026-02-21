// rift.generate plugin
// Defines the "generate" command and emits the "onGenerate" event
//
// Note: The Rift API is available globally without imports
// - defineCommand(name, handler)
// - emit(eventName, context)
// - on(eventName, handler)

// Define the generate command
defineCommand("generate", (ctx) => {
    console.log("Generating build system files...");

    // Emit the onGenerate event
    // Other plugins (like rift.go, rift.rust) will subscribe to this event
    emit("onGenerate", {
        workspaceRoot: ctx.workspaceRoot,
        packageName: ctx.packageName,
        packagePath: ctx.packagePath,
        packages: ctx.packages,
        config: ctx.config
    });

    console.log("Generation complete.");
});
