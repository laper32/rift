namespace Rift.Go.Workspace.Graph;

internal record GolangPackageGraphEdge(GolangPackageGraphNode Start, GolangPackageGraphNode End)
{
    public override string ToString()
    {
        return $"GolangPackageGraphEdge {{ Start = (Name = {Start.Name}, Version = {Start.Version}), End = (Name = {End.Name}, Version = {End.Version}) }}";
    }
}
