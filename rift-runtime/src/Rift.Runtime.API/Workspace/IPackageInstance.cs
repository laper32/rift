using System.Text.Json;

namespace Rift.Runtime.API.Workspace;

public interface IPackageInstance
{
    public string Name         { get; }
    public string ManifestPath { get; }
    JsonElement?  GetExtensionField(string name);
}