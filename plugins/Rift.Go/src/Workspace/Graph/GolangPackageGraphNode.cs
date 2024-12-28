namespace Rift.Go.Workspace.Graph;

internal record GolangPackageGraphNode(string Name, string Version)
{
    public bool Equals(string name, string version)
    {
        return Name.Equals(name, StringComparison.OrdinalIgnoreCase) && Version.Equals(version, StringComparison.OrdinalIgnoreCase);
    }

    public virtual bool Equals(GolangPackageGraphNode? rhs)
    {
        return rhs is not null && Equals(rhs.Name, rhs.Version);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Name, Version);
    }
}
