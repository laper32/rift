﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using System.Text.Json;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Plugin;
using Rift.Runtime.Workspace;

// ReSharper disable UnusedMember.Global

namespace Rift.Runtime.Scripting;

public class Dependencies
{
    public static void Add<T>(T dependency) where T: class, IPackageImportDeclarator
    {

        // 如果是false的话，就会去尝试插件那找
        if (!WorkspaceManagerInternal.Instance.AddDependencyForPackage(dependency))
        {
            Console.WriteLine($"Adding dependency => {JsonSerializer.Serialize(dependency)}");
            PluginManagerInternal.Instance.AddDependencyForPlugin(dependency);
        }

    }

    public static void Add<T>(IEnumerable<T> dependencies) where T : class, IPackageImportDeclarator
    {
        var packageImportDeclarators = dependencies as T[] ?? dependencies.ToArray();
        if (!WorkspaceManagerInternal.Instance.AddDependencyForPackage(packageImportDeclarators))
        {
            PluginManagerInternal.Instance.AddDependencyForPlugin(packageImportDeclarators);
        }
    }
}