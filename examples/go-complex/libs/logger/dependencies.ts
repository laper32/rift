// Logger library - uses workspace dependencies
// This demonstrates source: "workspace" to reference workspace-level dependencies

// Reference gin from workspace (defined in workspace/dependencies.ts)
addDependency({ name: "github.com/gin-gonic/gin", source: "workspace" });
