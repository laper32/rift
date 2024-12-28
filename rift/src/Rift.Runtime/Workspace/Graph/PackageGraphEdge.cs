﻿namespace Rift.Runtime.Workspace.Graph;

public record PackageGraphEdge(PackageGraphNode Start, PackageGraphNode End)
{
    public override string ToString()
    {
        return $"PackageGraphEdge {{ Start = (Name = {Start.Name}, Version = {Start.Version}), End = (Name = {End.Name}, Version = {End.Version}) }}";
    }
}