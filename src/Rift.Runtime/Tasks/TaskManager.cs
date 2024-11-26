// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Tasks;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

internal interface ITaskManagerInternal : ITaskManager, IInitializable;

internal class TaskManager : ITaskManagerInternal
{
    private readonly List<IRiftTask> _tasks;
    internal static  TaskManager     Instance { get; private set; } = null!;

    public TaskManager(InterfaceBridge bridge)
    {
        _tasks   = [];
        Instance = this;
    }

    public IRiftTask RegisterTask(string name)
    {
        if (_tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is { } task)
        {
            return task;
        }

        var ret = new RiftTask(name);
        _tasks.Add(ret);
        return ret;
    }

    public bool HasTask(string name) => _tasks.Any(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {
    }
}