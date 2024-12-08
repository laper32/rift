using Rift.Runtime.Workspace;

namespace Rift.Go.Workspace;

internal class GolangPackage(IPackageInstance package)
{
    internal IPackageInstance Instance { get; init; } = package;

    public string Name         { get; init; } = package.Name;
    public string ManifestPath { get; init; } = package.ManifestPath;
    public string Root         { get; init; } = package.Root;

    public Dictionary<string, PackageReference> Dependencies  => Instance.Dependencies;
    public PackageConfiguration                 Configuration => Instance.Configuration;
}