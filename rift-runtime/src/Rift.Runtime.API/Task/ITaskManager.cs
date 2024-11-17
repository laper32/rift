// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.API.Task;

public interface ITaskManager
{
    static ITaskManager Instance { get; protected set; } = null!;

    ITask? FindTask(string taskName);
    bool HasTask(string      taskName);
    void RegisterTask(string packageName, TaskManifest              taskManifest);
    void RegisterTask(string packageName, IEnumerable<TaskManifest> taskManifests);
}