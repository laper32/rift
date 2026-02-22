// packageB/tasks.ts
// Demonstrates cross-scope task dependency

// This task depends on packageA:build - a task from another package
tasks.register("test", {
    description: "Test package B (depends on packageA:build)",
    dependsOn: ["packageA:build"],  // Cross-scope dependency!
    run: (ctx) => {
        console.log("[packageB] Testing package B...");
        console.log("[packageB] This runs AFTER packageA:build completes");
    }
});

// Local dependency example (depends on build in same package)
tasks.register("package", {
    description: "Package packageB (depends on local build)",
    dependsOn: ["build"],
    run: (ctx) => {
        console.log("[packageB] Packaging...");
    }
});

tasks.register("build", {
    description: "Build package B",
    run: (ctx) => {
        console.log("[packageB] Building package B...");
    }
});
