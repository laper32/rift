// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Task;

namespace Rift.Runtime.Task;

internal interface ITaskManagerInternal : ITaskManager
{

}

internal class TaskManager : ITaskManagerInternal, IInitializable
{
    public TaskManager()
    {
        ITaskManager.Instance = this;
    }

    public bool Init()
    {
        IScriptManager.Instance.AddNamespace("Rift.Runtime.Task");
        return true;
    }

    public void Shutdown()
    {
        IScriptManager.Instance.RemoveNamespace("Rift.Runtime.Task");
    }

    private readonly List<ITask> _tasks = [];

    public void RegisterTask(string packageName, TaskManifest taskManifest)
    {
        if (HasTask(taskManifest.Name))
        {
            return;
        }

        _tasks.Add(new Task(packageName, taskManifest));
    }

    public void RegisterTask(string packageName, IEnumerable<TaskManifest> taskManifests)
    {
        foreach (var taskManifest in taskManifests)
        {
            RegisterTask(packageName, taskManifest);
        }
    }

    public bool HasTask(string taskName)
    {
        return _tasks.Any(x => x.Name.Equals(taskName, StringComparison.OrdinalIgnoreCase));
    }

    public ITask? FindTask(string taskName)
    {
        return _tasks.FirstOrDefault(x => x.Name.Equals(taskName, StringComparison.OrdinalIgnoreCase));
    }
}