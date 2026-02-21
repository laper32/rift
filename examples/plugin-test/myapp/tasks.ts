// Define tasks for myapp

// Simple build task
tasks.register("build", (task) => {
    task.description = "Build the application";
    task.do(() => {
        console.log("[myapp] Building application...");
        console.log("[myapp] Build complete!");
    });
});

// Test task that depends on build
tasks.register("test", (task) => {
    task.description = "Run tests";
    task.dependsOn("build");
    task.do(() => {
        console.log("[myapp] Running tests...");
        console.log("[myapp] All tests passed!");
    });
});

// Clean task (independent)
tasks.register("clean", (task) => {
    task.description = "Clean build artifacts";
    task.do(() => {
        console.log("[myapp] Cleaning build artifacts...");
        console.log("[myapp] Clean complete!");
    });
});

// CI task that depends on both build and test
tasks.register("ci", {
    description: "Run full CI pipeline",
    dependsOn: ["clean", "test"],
    run: () => {
        console.log("[myapp] CI pipeline complete!");
    }
});
