// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

public static partial class TaskConfigurationExtensions
{
    public static TaskConfiguration AddAction(this TaskConfiguration self, Action action)
    {
        return self.AddAction(_ => { action(); });
    }
}

public class TaskConfiguration(RiftTask task)
{
    internal RiftTask Instance { get; init; } = task;

    public TaskConfiguration OnInit(Action action)
    {

        return this;
    }

    public TaskConfiguration SetDeferException(bool value)
    {
        Instance.DeferExceptions = value;
        return this;
    }

    public TaskConfiguration SetErrorHandler(Func<Exception, ITaskContext, Task> predicate)
    {
        Instance.SetErrorHandler(predicate);
        return this;
    }

    public TaskConfiguration SetIsCommand(bool value)
    {
        if (value)
        {
            if (!Instance.Name.StartsWith("rift.", StringComparison.OrdinalIgnoreCase))
            {
                Tty.Warning($"Task `{Instance.Name}` must starts with `rift.` if you mark this task as command!");
                return this;
            }
        }

        Instance.IsCommand = value;
        return this;
    }

    public TaskConfiguration AddAction(Action<ITaskContext> action)
    {
        Instance.Actions.Add(context =>
        {
            action(context);
            return Task.CompletedTask;
        });
        return this;
    }

    public TaskConfiguration AddAction<TData>(Action<ITaskContext, TData> action) where TData : class
    {


        return this;
    }

    public TaskConfiguration SetDescription(string description)
    {
        Instance.Description = description;
        return this;
    }
}