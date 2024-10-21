namespace Rift.Runtime.API.Manifest;

public record FolderManifest(
    string Name,
    List<string> Members,
    List<string> Excludes
    );