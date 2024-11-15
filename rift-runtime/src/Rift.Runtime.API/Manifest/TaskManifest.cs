namespace Rift.Runtime.API.Manifest;

public record TaskManifest(
    string                 Name,
    bool IsCommand,
    string                 Description,
    string?                Parent,
    List<string>?          SubTasks,
    List<string>?          RunTasks,
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
