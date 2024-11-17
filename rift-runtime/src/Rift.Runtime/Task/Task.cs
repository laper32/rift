// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Task;

namespace Rift.Runtime.Task;

internal class Task(string packageName, TaskManifest manifest) : ITask
{
    public TaskManifest Manifest { get; init; } = manifest;

    public string Name => Manifest.Name;

    public string PackageName { get; init; } = packageName;

    public Action? Action { get; private set; }

    public bool    IsCommand   => Manifest.IsCommand;
    public string? Heading     => Manifest.Heading;
    public string? BeforeHelp  => Manifest.BeforeHelp;
    public string? AfterHelp   => Manifest.AfterHelp;
    public string? Description => Manifest.Description;
    public string? Parent      => Manifest.Parent;

    public List<string> RunTasks { get; init; } = manifest.RunTasks ?? [];

    /// <summary>
    /// 只存名字（aka: id)，需要考虑task没定义的时候，换句话说防呆。
    /// </summary>
    public List<string> SubTasks { get; init; } = manifest.SubTasks ?? [];

    public List<ITaskArg> Args { get; init; } = [];

    public void RegisterAction(Action action)
    {
        if (Action is not null)
        {
            return;
        }

        Action = action;
    }
}

internal class TaskArg(TaskArgManifest manifest) : ITaskArg
{
    public TaskArgManifest Manifest     { get; init; } = manifest;
    public string          Name         => Manifest.Name;
    public char?           Short        => Manifest.Short;
    public string?         Description  => Manifest.Description;
    public object?         Default      => Manifest.Default;
    public List<string>    ConflictWith => Manifest.ConflictWith ?? [];
    public string?         Heading      => Manifest.Heading;
}