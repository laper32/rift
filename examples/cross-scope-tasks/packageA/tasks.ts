// packageA/tasks.ts
// Demonstrates task that can be depended on by other packages

tasks.register("build", {
    description: "Build package A",
    run: (ctx) => {
        console.log("[packageA] Building package A...");
        console.log("[packageA] Workspace root:", ctx.workspaceRoot);
        console.log("[packageA] Package name:", ctx.packageName);
    }
});

tasks.register("clean", {
    description: "Clean package A",
    run: (ctx) => {
        console.log("[packageA] Cleaning package A...");
    }
});
