using System.Text.Json;

namespace Rift.Runtime.Abstractions.Workspace;

public interface IPackageInstance
{
    public string Name         { get; }
    public string ManifestPath { get; }

    JsonElement? GetExtensionField(string name);

    bool HasPlugin(string name);

    bool HasDependency(string name);
}