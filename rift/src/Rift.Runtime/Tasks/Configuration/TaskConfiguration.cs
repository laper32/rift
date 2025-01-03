﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.IO;
using Rift.Runtime.Tasks.Data;
using Rift.Runtime.Tasks.Fundamental;

namespace Rift.Runtime.Tasks.Configuration;

public static class TaskConfigurationExtensions
{
    public static TaskConfiguration AddAction(this TaskConfiguration self, Action action)
    {
        return self.AddAction(_ => { action(); });
    }

    public static TaskConfiguration AddOption<T>(
        this TaskConfiguration self,
        string name,
        Action<TaskOptionConfiguration<T>> predicate)
    {
        var cfg = new TaskOptionConfiguration<T>(name);
        predicate(cfg);
        var option = cfg.Build();

        var isExist = self
                .Instance
                .Options
                .Find(x => x.Name.Equals(option.Name, StringComparison.OrdinalIgnoreCase))
            is not null;

        if (isExist)
        {
            Tty.Warning($"Option `{option.Name}` already exists in `{self.Instance.Name}`");
            return self;
        }

        self.Instance.Options.Add(option);

        return self;
    }

    public static TaskConfiguration AddArgument<T>(
        this TaskConfiguration self,
        string name,
        Action<TaskArgumentConfiguration<T>> predicate)
    {
        var cfg = new TaskArgumentConfiguration<T>(name);
        predicate(cfg);
        var argument = cfg.Build();

        var isExist = self
                .Instance
                .Arguments
                .Find(x => x.Name.Equals(argument.Name, StringComparison.OrdinalIgnoreCase))
            is not null;

        if (isExist)
        {
            Tty.Warning($"Argument `{argument.Name}` already exists in `{self.Instance.Name}`");
            return self;
        }

        self.Instance.Arguments.Add(argument);

        return self;
    }

    public static TaskConfiguration Requires(this TaskConfiguration self, string taskName, bool required = true)
    {
        self.Instance.AddDependency(taskName, required);
        return self;
    }
}

public class TaskConfiguration(RiftTask task)
{
    internal RiftTask Instance { get; init; } = task;

    public TaskConfiguration SetDeferException(bool value)
    {
        Instance.DeferExceptions = value;
        return this;
    }

    public TaskConfiguration SetErrorHandler(Func<Exception, TaskContext, Task> predicate)
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

    public TaskConfiguration AddAction(Action<TaskContext> action)
    {
        Instance.Actions.Add(context =>
        {
            action(context);
            return Task.CompletedTask;
        });
        return this;
    }

    public TaskConfiguration AddAction<TData>(Action<TaskContext, TData> action) where TData : class
    {
        return this;
    }

    public TaskConfiguration SetDescription(string description)
    {
        Instance.Description = description;
        return this;
    }
}