// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


namespace Rift.Runtime.Task;

//internal class TaskManagerInternal : TaskManager, IInitializable
//{
//    public new static TaskManagerInternal Instance { get; private set; } = null!;
//    public TaskManagerInternal()
//    {
//        Instance = this;
//    }

//    public bool Init()
//    {
//        return true;
//    }

//    public void Shutdown()
//    {
//    }

//    private readonly List<ITask> _tasks = [];

//    public override void RegisterTask(string packageName, TaskManifest taskManifest)
//    {
//        if (HasTask(taskManifest.Name))
//        {
//            return;
//        }

//        var task = new Task(packageName, taskManifest);
//        var manifestArgs = taskManifest.Args ?? [];
//        var args = new List<TaskArg>();
//        manifestArgs.ForEach(x => args.Add(new TaskArg(x)));
//        task.Args.AddRange(args);

//        _tasks.Add(task);
//    }

//    public override void RegisterTask(string packageName, IEnumerable<TaskManifest> taskManifests)
//    {
//        foreach (var taskManifest in taskManifests)
//        {
//            RegisterTask(packageName, taskManifest);
//        }
//    }

//    public override bool HasTask(string taskName)
//    {
//        return _tasks.Any(x => x.Name.Equals(taskName, StringComparison.OrdinalIgnoreCase));
//    }

//    public override ITask? FindTask(string taskName)
//    {
//        return _tasks.FirstOrDefault(x => x.Name.Equals(taskName, StringComparison.OrdinalIgnoreCase));
//    }

//    public List<UserDefinedCommand> GetUserDefinedCommands()
//    {
//        var ret = new List<UserDefinedCommand>();
//        _tasks.ForEach(task =>
//        {
//            if (!task.IsCommand)
//            {
//                return;
//            }

//            var args = new List<UserDefinedCommandArg>();
//            task.Args.ForEach(arg => args.Add(new UserDefinedCommandArg(
//                arg.Name,
//                arg.Short,
//                arg.Description,
//                arg.Default,
//                arg.ConflictWith,
//                arg.Heading
//            )));
            
//            var subCommands = new List<string>();
//            task.SubTasks.ForEach(taskName =>
//            {
//                if (FindTask(taskName) is not { } taskInstance)
//                {
//                    return;
//                }

//                if (!taskInstance.IsCommand)
//                {
//                    return;
//                }

//                subCommands.Add(taskName);
//            });

//            var command = new UserDefinedCommand(
//                Name: task.Name,
//                About: task.Description,
//                BeforeHelp: task.BeforeHelp,
//                AfterHelp: task.AfterHelp,
//                Parent: task.Parent,
//                Subcommands: subCommands,
//                PackageName: task.PackageName,
//                Args: args
//            );

//            ret.Add(command);
//        });

//        return ret;
//    }
//}