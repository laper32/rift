namespace Rift.Runtime.Workspace;

internal class PackageInstance(IMaybePackage package)
{
    public IMaybePackage Value { get; init; } = package;

    public Dictionary<string, object> Metadata     { get; init; } = [];
    public Dictionary<string, object> Dependencies { get; init; } = [];
    public Dictionary<string, object> Plugins     { get; init; } = [];
}