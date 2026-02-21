// API service dependencies

// Inherit chi from the parent folder (services/)
addDependency({ name: "github.com/go-chi/chi/v5", source: "inherit" });

// Reference workspace dependency
addDependency({ name: "github.com/stretchr/testify", source: "workspace" });

// External dependency
addDependency({ name: "github.com/gorilla/websocket", version: "1.5.0" });

// Reference internal workspace packages
addDependency({ name: "common", source: "workspace" });
addDependency({ name: "logger", source: "workspace" });
