// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks.Structuring;

internal class TaskGraphEdge(string start, string end)
{
    public string Start { get; } = start;
    public string End { get; } = end;
}