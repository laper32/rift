// Frontend service dependencies

// Inherit chi from the parent folder (services/)
addDependency({ name: "github.com/go-chi/chi/v5", source: "inherit" });

// Reference workspace dependency
addDependency({ name: "github.com/gin-gonic/gin", source: "workspace" });

// Internal workspace packages
addDependency({ name: "logger", source: "workspace" });
