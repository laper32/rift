// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Task;

public interface ITaskManager
{
    public static ITaskManager Instance { get; protected set; } = null!;
}