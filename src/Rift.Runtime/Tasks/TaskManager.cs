// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


using Rift.Runtime.Scripting;

namespace Rift.Runtime.Tasks;

public sealed class TaskManager
{
    private static TaskManager _instance = null!;
    private readonly List<RiftTask> _tasks;

    public TaskManager()
    {
        _tasks = [];
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

    public static void RunTask(string name)
    {
        if (_instance._tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is not { } task)
        {
            return;
        }

        task.Invoke(new TaskContext()).ConfigureAwait(false).GetAwaiter().GetResult();

        //task.Invoke(new TaskContext());
    }

    internal static void RunTask(string name, TaskContext context)
    {
        if (_instance._tasks.Find(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)) is not { } task)
        {
            return;
        }

        task.Invoke(context).ConfigureAwait(false).GetAwaiter().GetResult();
    }

    internal static List<string> GetMarkedAsCommandTasks()
    {
        return (from RiftTask task in _instance._tasks where task.IsCommand select task.Name).ToList();
    }
}