// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using Rift.Runtime.Workspace;

namespace Rift.Runtime.Scripting;

// ReSharper disable UnusedMember.Global
public class Plugins
{
    /// <summary>
    ///     为包注册一个需要用到的插件<br />
    ///     <remarks>
    ///         1. 只能在脚本中调用该函数<br />
    ///         2. 只有workspace中的包可以调用该函数<br />
    ///     </remarks>
    /// </summary>
    /// <param name="plugin"> </param>
    public static void Add(PackageReference plugin)
    {
        WorkspaceManager.AddPluginForPackage(plugin);
    }

    /// <summary>
    ///     为包注册多个需要用到的插件<br />
    ///     <remarks>
    ///         1. 只能在脚本中调用该函数<br />
    ///         2. 只有workspace中的包可以调用该函数<br />
    ///     </remarks>
    /// </summary>
    /// <param name="plugins"> </param>
    public static void Add(IEnumerable<PackageReference> plugins)
    {
        WorkspaceManager.AddPluginForPackage(plugins);
    }
}