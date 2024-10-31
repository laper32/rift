// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// ReSharper disable UnusedMember.Global

namespace Rift.Runtime.Task;

public static class Tasks
{
    public delegate void TaskAction();
    public static void Implement(string taskName, TaskAction action)
    {
        action();
    }
}