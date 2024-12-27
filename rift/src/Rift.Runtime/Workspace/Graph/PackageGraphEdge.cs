namespace Rift.Runtime.Workspace.Graph;

internal class PackageGraphEdge(PackageGraphNode start, PackageGraphNode end)
{
    public PackageGraphNode Start { get; init; } = start;
    public PackageGraphNode End   { get; init; } = end;
}
