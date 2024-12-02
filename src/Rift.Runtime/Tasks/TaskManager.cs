﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

public interface ITaskManager
{
    /// <summary>
    /// 注册一个任务 <br/>
    /// <remarks>
    /// 如果该任务已经存在，将返回已经存在的任务。 <br/>
    /// </remarks>
    /// </summary>
    /// <param name="name">任务名</param>
    /// <param name="predicate">任务配置</param>">
    /// <returns>想获取的任务</returns>
    IRiftTask RegisterTask(string name, Action<ITaskConfiguration> predicate);

    /// <summary>
    /// 找到你想要的任务
    /// </summary>
    /// <param name="name">对应的任务名</param>
    /// <returns></returns>
    IRiftTask? FindTask(string name);

    /// <summary>
    /// 判断该任务是否存在
    /// </summary>
    /// <param name="name">任务名</param>
    /// <returns>想获取的任务</returns>
    bool HasTask(string name);

    void RunTask(string name);
}
internal interface ITaskManagerInternal : ITaskManager, IInitializable
{
    List<string> GetMarkedAsCommandTasks();
}

internal class TaskManager : ITaskManagerInternal
{
    private readonly List<IRiftTask> _tasks;
    internal static  TaskManager     Instance { get; private set; } = null!;
    private readonly InterfaceBridge _bridge;
    public TaskManager(InterfaceBridge bridge)
    {
        _tasks   = [];
        _bridge  = bridge;
        Instance = this;
    }

    public IRiftTask RegisterTask(string name, Action<ITaskConfiguration> predicate)
    {
        TaskConfiguration cfg;
        if (_tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is {} task)
        {
            cfg = new TaskConfiguration((RiftTask)task);
            predicate(cfg);

            return task;
        }

        var ret = new RiftTask(name);
        cfg = new TaskConfiguration(ret);
        predicate(cfg);
        _tasks.Add(ret);

        return ret;
    }

    public IRiftTask? FindTask(string name) =>
        _tasks.FirstOrDefault(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

    public bool HasTask(string name) => _tasks.Any(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

    public void RunTask(string name)
    {
        if (_tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is not RiftTask task)
        {
            return;
        }

        var context = new TaskContext(_bridge);

        try
        {
            ExecuteTask(task, context).GetAwaiter().GetResult();
        }
        catch (Exception e)
        {
            task.ErrorHandler?.Invoke(e, context);
        }
    }

    private async Task ExecuteTask(RiftTask task, TaskContext context)
    {
        await task.Invoke(context);
    }

    public List<string> GetMarkedAsCommandTasks()
    {
        return (from RiftTask task in _tasks where task.IsCommand select task.Name).ToList();
    }

    public bool Init()
    {
        _bridge.ScriptManager.AddNamespace("Rift.Runtime.Tasks");
        return true;
    }

    public void Shutdown()
    {
        _bridge.ScriptManager.RemoveNamespace("Rift.Runtime.Tasks");
    }
}