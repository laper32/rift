// Task definitions for the app package
declare global {
    const tasks: {
        register: (
            name: string,
            options: {
                description?: string;
                dependsOn?: string | string[];
                dependencies?: string | string[];
                command?: boolean;
                asCommand?: boolean;
                run?: () => void;
                do?: () => void;
            }
        ) => void;
    };
    const dependencies: {
        add: (dep: string | { name: string; version?: string }) => void;
    };
}
export {};

// Register tasks using object configuration (camelCase)
tasks.register("app.build", {
    description: "Build the application",
    command: true,
    run: () => {
        console.log("Building app...");
    }
});

tasks.register("app.test", {
    description: "Run tests for the application",
    dependsOn: ["app.build"],
    command: true,
    run: () => {
        console.log("Testing app...");
    }
});

tasks.register("app.clean", {
    description: "Clean build artifacts",
    // Internal task, not exposed as CLI command
    command: false,
    run: () => {
        console.log("Cleaning app...");
    }
});
