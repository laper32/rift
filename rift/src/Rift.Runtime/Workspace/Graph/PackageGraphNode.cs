namespace Rift.Runtime.Workspace.Graph;

internal class PackageGraphNode
{
    public string Name    { get; init; }
    public string Version { get; init; }

    public PackageGraphNode(string packageName, string packageVersion)
    {
        ArgumentException.ThrowIfNullOrEmpty(packageName);
        ArgumentException.ThrowIfNullOrEmpty(packageVersion);

        Name    = packageName;
        Version = packageVersion;
    }

    private bool Equals(string name, string version)
    {
        return Name.Equals(name, StringComparison.OrdinalIgnoreCase) && Version.Equals(version, StringComparison.OrdinalIgnoreCase);
    }

    public override bool Equals(object? obj)
    {
        return obj is PackageGraphNode rhs && Equals(rhs.Name, rhs.Version);
    }

    protected bool Equals(PackageGraphNode other)
    {
        return Name == other.Name && Version == other.Version;
    }


    public override int GetHashCode()
    {
        return HashCode.Combine(Name, Version);
    }
}
