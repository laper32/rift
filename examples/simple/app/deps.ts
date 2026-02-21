// App dependencies script
// Demonstrates the enhanced PackageReference API

declare global {
    const Dependencies: {
        add: (dep: string | { name: string; version?: string }) => void;
    };
}

export {};

// ===== Simple workspace-local dependencies =====
// App depends on utils (workspace-local)
Dependencies.add("utils");

// ===== External dependencies with versions =====
// String form: "name@version"
Dependencies.add("lodash@4.17.21");
Dependencies.add("react@18.2.0");

// ===== Scoped/namespaced packages =====
// Using slash separator (npm style)
Dependencies.add("angular/core@16.0.0");
Dependencies.add("vue/router@4.2.0");

// Using dot separator (Java/C# style)
Dependencies.add("org.apache.commons");
Dependencies.add("com.google.guava@31.1.0");

// ===== Object form with explicit name/version =====
Dependencies.add({
    name: "typescript",
    version: "5.0.0"
});

// ===== Full repository-style references =====
Dependencies.add("github.com/user/repo@v1.0.0");

console.log("App dependencies loaded with PackageReference API");
