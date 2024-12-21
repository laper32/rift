// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Scripts.Managers;
using Rift.Runtime.Tasks.Configuration;
using Rift.Runtime.Tasks.Fundamental;
using Rift.Runtime.Tasks.Reporting;
using Rift.Runtime.Tasks.Scheduling;
using TaskScheduler = Rift.Runtime.Tasks.Scheduling.TaskScheduler;

namespace Rift.Runtime.Tasks.Managers;

public sealed class TaskManager
{
    private static   TaskManager    _instance = null!;
    private readonly TaskExecutor   _taskExecutor;
    private readonly List<RiftTask> _tasks;

    private readonly TaskScheduler _taskScheduler;

    public TaskManager()
    {
        var services = new ServiceCollection();
        services.AddSingleton<TaskScheduler>();
        services.AddSingleton<TaskExecutor>();

        var provider = services.BuildServiceProvider();
        _taskScheduler = provider.GetRequiredService<TaskScheduler>();
        _taskExecutor  = provider.GetRequiredService<TaskExecutor>();

        _tasks    = [];
        _instance = this;
    }

    internal static bool Init()
    {
        ScriptManager.AddNamespace("Rift.Runtime.Tasks");
        return true;
    }

    internal static void Shutdown()
    {
        ScriptManager.RemoveNamespace("Rift.Runtime.Tasks");
    }

    /// <summary>
    ///     注册一个任务 <br />
    ///     <remarks>
    ///         如果该任务已经存在，将返回已经存在的任务。 <br />
    ///     </remarks>
    /// </summary>
    /// <param name="name"> 任务名 </param>
    /// <param name="predicate"> 任务配置 </param>
    /// ">
    /// <returns> 想获取的任务 </returns>
    public static RiftTask RegisterTask(string name, Action<TaskConfiguration> predicate)
    {
        TaskConfiguration cfg;
        if (_instance._tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is { } task)
        {
            cfg = new TaskConfiguration(task);
            predicate(cfg);

            return task;
        }

        var ret = new RiftTask(name);
        cfg = new TaskConfiguration(ret);
        predicate(cfg);
        _instance._tasks.Add(ret);

        return ret;
    }

    /// <summary>
    ///     找到你想要的任务
    /// </summary>
    /// <param name="name"> 对应的任务名 </param>
    /// <returns> </returns>
    public static RiftTask? FindTask(string name)
    {
        return _instance._tasks.FirstOrDefault(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));
    }

    /// <summary>
    ///     判断该任务是否存在
    /// </summary>
    /// <param name="name"> 任务名 </param>
    /// <returns> 想获取的任务 </returns>
    public static bool HasTask(string name)
    {
        return _instance._tasks.Any(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));
    }

    public static void ScheduleTask(string name)
    {
        _instance._taskScheduler.Enqueue(name);
    }

    // TODO: 在没想好怎么做泛型参数支持之前，先用着TaskContext，且不对外。
    internal static void ScheduleTask(string name, TaskContext context)
    {
        _instance._taskScheduler.Enqueue(name, context);
    }

    internal static void RunTasks()
    {
        var report = _instance._taskExecutor.ExecuteTasks(_instance._tasks);
        _instance.SummaryTasks(report);
    }

    private void SummaryTasks(TaskReport report)
    {
        foreach (var recipe in report)
        {
            Console.WriteLine(
                $" => {recipe.TaskName} used {recipe.Duration} seconds (Execution status: {recipe.ExecutionStatus})");
        }
    }

    internal static List<string> GetMarkedAsCommandTasks()
    {
        return (from RiftTask task in _instance._tasks where task.IsCommand select task.Name).ToList();
    }
}