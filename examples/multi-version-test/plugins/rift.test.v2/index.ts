// rift.test.v2 plugin - version 2.0.0
// Test plugin for multi-version isolation

// Subscribe to test event
on("onTest", (ctx) => {
    console.log("[rift.test.v2] Handler v2.0.0 executed!");
    console.log("[rift.test.v2] Context:", JSON.stringify(ctx));
});

// Register a test command
defineCommand("test-v2", (ctx) => {
    console.log("[rift.test.v2] Command v2.0.0 executed!");
    emit("onTest", { from: "v2", message: "Hello from v2.0.0" });
});
