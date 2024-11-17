// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.InteropServices;
using System.Text;
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

        var task = new Task(packageName, taskManifest);
        var manifestArgs = taskManifest.Args ?? [];
        var args = new List<TaskArg>();
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
        string? About,
        string? BeforeHelp,
        string? AfterHelp,
        string? Parent,
        List<string> SubTasks,
        List<string> RunTasks,
        string PackageName,
        List<CommandedTaskArg> Args);

    private record CommandedTaskArg(
        string Name,
        char? Short,
        string? Description,
        object? Default,
        List<string> ConflictWith,
        string? Heading);

    private List<CommandedTask> ExportMarkedAsCommandTasks()
    {
        var ret = new List<CommandedTask>();

        _tasks.ForEach(task =>
        {
            var args = new List<CommandedTaskArg>();
            task.Args.ForEach(arg => args.Add(new CommandedTaskArg(arg.Name, arg.Short, arg.Description, arg.Default, arg.ConflictWith, arg.Heading)));
            ret.Add(
                new CommandedTask(
                    Name: task.Name,
                    About: task.Description,
                    BeforeHelp: task.BeforeHelp,
                    AfterHelp: task.AfterHelp,
                    Parent: task.Parent,
                    SubTasks: task.SubTasks,
                    RunTasks: task.RunTasks,
                    PackageName: task.PackageName,
                    Args: args
                )
            );
        });
        return ret;
    }


    [UnmanagedCallersOnly]
    public static unsafe sbyte* GetTasksExport()
    {
        var taskManager = (TaskManager)ITaskManager.Instance;
        var commands    = taskManager.ExportMarkedAsCommandTasks();
        var commandsStr = JsonSerializer.Serialize(commands, new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
        });

        var bytes = Encoding.UTF8.GetBytes(commandsStr);
        var sBytes = Array.ConvertAll(bytes, Convert.ToSByte);

        fixed (sbyte* p = sBytes)
        {
            return p;
        }
    }
}