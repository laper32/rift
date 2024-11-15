namespace Rift.Runtime.API.Manifest;

public record TaskManifest(
    string                 Name,
    string                 Description,
    string?                Parent,
    List<TaskArgManifest>? Args
);

public record TaskArgManifest(
    string        Name,
    char?       Short,
    string?       Description,
    object?       Default,
    List<string>? ConflictWith,
    string?       Heading
);
