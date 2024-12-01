// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using System.Text.Json.Serialization;
using Rift.Runtime.Abstractions.Scripting;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Scripting;

public class Plugin : IPackageImportDeclarator
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

    [JsonIgnore]
    private bool _refWorkspace;

    /// <summary>
    /// 指定该包使用根目录下的声明. <br/>
    ///
    /// 该标记拥有最高优先级. <br />
    /// 如果你的项目比较复杂(如Workspace-Project-Target结构), 则请使用 <see cref="Ref"/>声明具体需要使用的项目.
    /// </summary>
    /// <returns>Instance this</returns>
    public Plugin RefWorkspace()
    {
        if (_refWorkspace)
        {
            return this;
        }

        Attributes.Add("RefWorkspace", true);
        _refWorkspace = true;

        return this;
    }

    public Plugin Ref(string packageName)
    {
        Attributes.Add("Ref", packageName);
        return this;
    }
}

// ReSharper disable UnusedMember.Global

// TODO: CommandLine Remake
public class Plugins
{
    public static void Add(Plugin plugin)
    {
        WorkspaceManager.Instance.AddPluginForPackage(plugin);
    }

    public static void Add(IEnumerable<Plugin> plugins)
    {
        WorkspaceManager.Instance.AddPluginForPackage(plugins);
    }
}