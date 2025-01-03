﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using Rift.Runtime.Plugins.Managers;
using Rift.Runtime.Workspace.Fundamental;
using Rift.Runtime.Workspace.Managers;

// ReSharper disable UnusedMember.Global

namespace Rift.Runtime.Scripts.Scripting;

public class Dependencies
{
    /// <summary>
    ///     添加一个依赖 <br />
    ///     <remarks>
    ///         该函数只允许脚本调用！
    ///     </remarks>
    /// </summary>
    /// <param name="dependency"> </param>
    public static void Add(PackageReference dependency)
    {
        // 如果是false的话，就会去尝试插件那找
        if (WorkspaceManager.AddDependencyForPackage(dependency))
        {
        }

        PluginManager.AddDependencyForPlugin(dependency);
    }

    /// <summary>
    ///     添加多个依赖 <br />
    ///     <remarks>
    ///         该函数只允许脚本调用！
    ///     </remarks>
    /// </summary>
    /// <param name="dependencies"> </param>
    public static void Add(IEnumerable<PackageReference> dependencies)
    {
        var references = dependencies as PackageReference[] ?? dependencies.ToArray();
        if (WorkspaceManager.AddDependencyForPackage(references))
        {
        }

        PluginManager.AddDependencyForPlugin(references);
    }
}