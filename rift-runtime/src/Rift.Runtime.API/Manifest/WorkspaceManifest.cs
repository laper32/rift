namespace Rift.Runtime.API.Manifest;

public record WorkspaceManifest(
    List<string> Members,
    List<string> Excludes,
    string PluginScriptPath,
    string MetadataScriptPath,
    string DependenciesScriptPath
    );