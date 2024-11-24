// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Scripting;

internal class ScriptContext(InterfaceBridge bridge, string path) : IScriptContext
{
    public string Path { get; init; } = path;

    // 举个例子：C:/Users/user/MyScript.csx，那么所在的ScriptDir就是C:/Users/user
    // 这里的核心目的是得知该脚本文件所在的文件夹，方便操作。
    public string Location { get; init; } = Directory.GetParent(path)!.FullName;

    public string            Text             { get; init; } = File.ReadAllText(path);
    public IRuntime          Runtime          => bridge.Runtime;
    public IWorkspaceManager WorkspaceManager => bridge.WorkspaceManager;
}