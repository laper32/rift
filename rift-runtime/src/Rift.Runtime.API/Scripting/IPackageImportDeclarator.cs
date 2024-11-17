// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Scripting;

public interface IPackageImportDeclarator
{
    /// <summary>
    /// 不管是什么依赖，首先得有名字。。
    /// </summary>
    public string Name { get; }
}