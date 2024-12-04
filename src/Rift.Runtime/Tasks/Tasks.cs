// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

// ReSharper disable UnusedMember.Global
public class Tasks
{
    public static void Register(string name, Action<ITaskConfiguration> predicate)
    {
        TaskManager.Instance.RegisterTask(name, predicate);
    }
}