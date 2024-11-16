// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.InteropServices;
using System.Text;
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

        var task         = new Task(packageName, taskManifest);
        var manifestArgs = taskManifest.Args ?? [];
        var args         = new List<TaskArg>();
        manifestArgs.ForEach(x => args.Add(new TaskArg(x)));
        task.Args.AddRange(args);

        _tasks.Add(task);
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

    private record CommandedTask(
        string Name,
        string About,
        string BeforeHelp,
        string AfterHelp,
        string Parent,
        List<string> SubTasks,
        List<string> RunTasks,
        string PackageName,
        List<CommandedTaskArg> Args);

    private record CommandedTaskArg(
        string Name,
        char Short,
        string Description,
        object Default,
        List<string> ConflictWith,
        string Heading);


    [UnmanagedCallersOnly]
    public static unsafe sbyte* GetTasksExport()
    {

        const string str = """
                               {
                                   "Id": "123",
                                   "DateOfRegistration": "2012-10-21T00:00:00+05:30",
                                   "Status": 0
                               }
                           """;
        var          bytes  = Encoding.UTF8.GetBytes(str);
        var          sBytes = Array.ConvertAll(bytes, Convert.ToSByte);

        fixed (sbyte* p = sBytes)
        {
            return p;
        }
    }
}