// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;



public static class TaskConfigurationExtensions
{
    public static TaskConfiguration AddAction(this TaskConfiguration configuration, Action action)
    {
        return configuration.AddAction(_ => { action(); });
    }
}

public class TaskConfiguration(RiftTask task)
{
    public TaskConfiguration SetDeferException(bool value)
    {
        task.DeferExceptions = value;
        return this;
    }

    public TaskConfiguration SetErrorHandler(Func<Exception, ITaskContext, Task> predicate)
    {
        task.SetErrorHandler(predicate);
        return this;
    }

    public TaskConfiguration SetIsCommand(bool value)
    {
        if (value)
        {
            if (!task.Name.StartsWith("rift.", StringComparison.OrdinalIgnoreCase))
            {
                Tty.Warning($"Task `{task.Name}` must starts with `rift.` if you mark this task as command!");
                return this;
            }
        }

        task.IsCommand = value;
        return this;
    }

    public TaskConfiguration AddAction(Action<ITaskContext> action)
    {
        task.Actions.Add(context =>
        {
            action(context);
            return Task.CompletedTask;
        });
        return this;
    }

    public TaskConfiguration SetDescription(string description)
    {
        task.Description = description;
        return this;
    }
}