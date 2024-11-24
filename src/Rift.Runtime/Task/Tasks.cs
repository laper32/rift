// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// ReSharper disable UnusedMember.Global

using Rift.Runtime.API.Task;

namespace Rift.Runtime.Task;

public static class Tasks
{
    public static void Implement(string taskName, Action action)
    {
        if (TaskManager.Instance.FindTask(taskName) is { } task)
        {
            task.RegisterAction(action);
        }
    }
}