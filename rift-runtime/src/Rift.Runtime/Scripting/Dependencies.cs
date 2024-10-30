// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

// 为什么我们把暴露给脚本的三兄弟都放在本体里而不是放在API包里？
// 原因只有一个：暴露给脚本的API不能设计的过于复杂，且很多时候脚本是没办法看到dll里面有什么API的。

using Rift.Runtime.API.Scripting;

// ReSharper disable UnusedMember.Global

namespace Rift.Runtime.Scripting;

public static class Dependencies
{
    public static void Add<T>(T dependency) where T: class, IPackageDependency
    {

    }
}