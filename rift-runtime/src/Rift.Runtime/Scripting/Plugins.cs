// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using Rift.Runtime.API.Workspace;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Scripting;

public class Plugin
{
    public string                     Name       { get; init; }
    public string                     Version    { get; set; }
    public Dictionary<string, object> Attributes { get; init; }

    public Plugin(string name)
    {
        Name       = name;
        Version    = "";
        Attributes = [];
    }

    public Plugin(string name, string version)
    {
        Name       = name;
        Version    = version;
        Attributes = [];
    }
}

// ReSharper disable UnusedMember.Global

public static class Plugins
{
    public static void Add(Plugin plugin)
    {
        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        workspaceManager.AddPluginForPackage(plugin);
    }

    public static void Add(IEnumerable<Plugin> plugins)
    {
        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        workspaceManager.AddPluginForPackage(plugins);
    }
}