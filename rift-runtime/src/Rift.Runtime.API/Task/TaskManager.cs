// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.API.Task;

public abstract class TaskManager
{
    public static TaskManager Instance { get; protected set; } = null!;

    protected TaskManager()
    {
        Instance = this;
    }

    public abstract ITask? FindTask(string     taskName);
    public abstract bool   HasTask(string      taskName);
    public abstract void   RegisterTask(string packageName, TaskManifest              taskManifest);
    public abstract void   RegisterTask(string packageName, IEnumerable<TaskManifest> taskManifests);
}