// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Task;

public interface ITask
{
    public string         Name        { get; }
    public bool           IsCommand   { get; }
    public string?        Heading     { get; }
    public string?        BeforeHelp  { get; }
    public string?        AfterHelp   { get; }
    public string?        Description { get; }
    public string?        Parent      { get; }
    public List<string>   SubTasks    { get; }
    public List<string>   RunTasks    { get; }
    public string         PackageName { get; }
    public Action?        Action      { get; }
    public List<ITaskArg> Args        { get; }

    void                         RegisterAction(Action action);
}

public interface ITaskArg
{
    public string       Name         { get; }
    public char?        Short        { get; }
    public string?      Description  { get; }
    public object?      Default      { get; }
    public List<string> ConflictWith { get; }
    public string?      Heading      { get; }
}
