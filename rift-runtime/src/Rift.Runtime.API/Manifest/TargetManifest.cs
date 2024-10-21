namespace Rift.Runtime.API.Manifest;

public record TargetManifest(
    string Name,
    string Type,
    string PluginScriptPath,
    string DependenciesScriptPath,
    string MetadataScriptPath
    );