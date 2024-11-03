namespace Rift.Runtime.API.Manifest;

public record PluginManifest(
    string       Name,
    List<string> Authors,
    string       Version,
    string?      Description,
    string?      Metadata,
    string?      Dependency
);
