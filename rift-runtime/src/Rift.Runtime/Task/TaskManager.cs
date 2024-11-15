﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

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

    public void AnalyzeTasks()
    {
        foreach (var subTaskInterface in _tasks
                     .Where(taskInterface => taskInterface.IsCommand)
                     .Select(taskInterface => (Task)taskInterface)
                     .Select(task => task.SubTasks)
                     .SelectMany(subTasks => subTasks.Select(FindTask)))
        {
            if (subTaskInterface is not Task subTaskImpl)
            {
                continue;
            }

            subTaskImpl.IsCommand = true;
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