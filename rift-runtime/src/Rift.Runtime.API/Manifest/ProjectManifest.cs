namespace Rift.Runtime.API.Manifest;

public record ProjectManifest(
    string Name,
    List<string> Authors,
    string Version,
    string Description,
    string PluginScriptPath,
    string DependenciesScriptPath,
    string MetadataScriptPath,

    List<string> Members,
    List<string> Excludes
    );