// app/tasks.ts
// Import functions from the mathPlugin using rift: protocol

// Use ES module import syntax for rift: imports
import { add, multiply, PI } from "rift:mathPlugin";

tasks.register("test-math", {
    description: "Test the rift: import by using mathPlugin functions",
    run: (ctx) => {
        console.log("[app] Testing rift: import functionality...");

        // Use imported functions directly
        const result1 = add(5, 3);
        console.log("[app] add(5, 3) =", result1);

        const result2 = multiply(4, 7);
        console.log("[app] multiply(4, 7) =", result2);

        console.log("[app] PI constant:", PI);

        // Test that we got the real values
        if (result1 === 8 && result2 === 28 && PI === 3.14159) {
            console.log("[app] SUCCESS: All imports work correctly!");
        } else {
            console.log("[app] ERROR: Imports didn't work as expected");
        }
    }
});
