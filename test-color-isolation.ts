// Test script to verify color isolation
// This simulates loading two versions of the same plugin

console.log("=== Testing Color Isolation ===\n");

// Simulate plugin v1.0.0 in packageA scope
console.log("1. Loading plugin@1.0.0 in packageA scope...");
globalThis._rift_current_color = "myplugin@1.0.0:packageA";

// Register event handler for v1
on("onTest", (ctx) => {
    console.log("[v1.0.0/packageA] Handler executed!");
    console.log("[v1.0.0/packageA] Color:", globalThis._rift_current_color);
    console.log("[v1.0.0/packageA] Received:", JSON.stringify(ctx));
});

console.log("   Registered handler with color:", globalThis._rift_current_color);

// Simulate plugin v2.0.0 in packageB scope
console.log("\n2. Loading plugin@2.0.0 in packageB scope...");
globalThis._rift_current_color = "myplugin@2.0.0:packageB";

// Register event handler for v2
on("onTest", (ctx) => {
    console.log("[v2.0.0/packageB] Handler executed!");
    console.log("[v2.0.0/packageB] Color:", globalThis._rift_current_color);
    console.log("[v2.0.0/packageB] Received:", JSON.stringify(ctx));
});

console.log("   Registered handler with color:", globalThis._rift_current_color);

// Test 1: Emit to all (no filter)
console.log("\n=== Test 1: Emit to ALL handlers ===");
emit("onTest", { message: "Hello to all!" });

// Test 2: Emit only to packageA scope
console.log("\n=== Test 2: Emit to packageA scope only ===");
emit("onTest", { message: "Hello packageA!" }, { scope: "packageA" });

// Test 3: Emit only to packageB scope
console.log("\n=== Test 3: Emit to packageB scope only ===");
emit("onTest", { message: "Hello packageB!" }, { scope: "packageB" });

// Test 4: Emit to exact color (v1)
console.log("\n=== Test 4: Emit to exact color myplugin@1.0.0:packageA ===");
emit("onTest", { message: "Hello v1!" }, { color: "myplugin@1.0.0:packageA" });

// Test 5: Emit to exact color (v2)
console.log("\n=== Test 5: Emit to exact color myplugin@2.0.0:packageB ===");
emit("onTest", { message: "Hello v2!" }, { color: "myplugin@2.0.0:packageB" });

console.log("\n=== Test Complete ===");
