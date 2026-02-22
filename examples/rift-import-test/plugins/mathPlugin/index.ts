// mathPlugin/index.ts
// A plugin that exports utility functions

// Define the functions to export
function add(x: number, y: number): number {
    return x + y;
}

function multiply(x: number, y: number): number {
    return x * y;
}

function divide(x: number, y: number): number {
    if (y === 0) {
        throw new Error("Division by zero");
    }
    return x / y;
}

// Constants
const PI = 3.14159;
const E = 2.71828;

// Register exports so other packages can import them via "rift:mathPlugin"
rift.registerExports("mathPlugin", {
    add,
    multiply,
    divide,
    PI,
    E,
});

// Also register a command (plugin can do both)
defineCommand("math-test", () => {
    console.log("[mathPlugin] Test command executed!");
    console.log("PI =", PI);
    console.log("2 + 3 =", add(2, 3));
});

// Subscribe to events (plugin can do both)
on("onMath", (ctx) => {
    console.log("[mathPlugin] onMath event:", JSON.stringify(ctx));
});
