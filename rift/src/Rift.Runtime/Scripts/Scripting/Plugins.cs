// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Workspace.Fundamental;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Scripts.Scripting;

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