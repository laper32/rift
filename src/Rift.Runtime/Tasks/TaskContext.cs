// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

internal class TaskContext : ITaskContext
{
    public ITaskArguments Arguments { get; }
}

public interface ITaskContext
{
    ITaskArguments Arguments { get; }
}