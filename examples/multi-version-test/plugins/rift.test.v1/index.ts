// rift.test.v1 plugin - version 1.0.0
// Test plugin for multi-version isolation

// Subscribe to test event
on("onTest", (ctx) => {
    console.log("[rift.test.v1] Handler v1.0.0 executed!");
    console.log("[rift.test.v1] Context:", JSON.stringify(ctx));
});

// Register test commands
defineCommand("test-v1", () => {
    console.log("[rift.test.v1] Command v1.0.0 executed!");
    console.log("[rift.test.v1] Emitting to default scope (should only trigger v1)");
    emit("onTest", { from: "v1", message: "Hello from v1.0.0" });
});

// Test cross-scope emit - trigger all handlers
defineCommand("test-all", () => {
    console.log("[rift.test.v1] test-all command executed!");
    console.log("[rift.test.v1] Emitting to ALL scopes (should trigger both v1 and v2)");
    emit("onTest", { from: "test-all", message: "Hello to all versions!" }, { scope: "all" });
});

// Test exact color targeting
defineCommand("test-target-v2", () => {
    console.log("[rift.test.v1] test-target-v2 command executed!");
    console.log("[rift.test.v1] Emitting specifically to v2 (should only trigger v2)");
    emit("onTest", { from: "v1-targeting-v2", message: "Hello from v1 to v2" }, { color: "rift.test.v2@2.0.0:rift.test.v2" });
});
