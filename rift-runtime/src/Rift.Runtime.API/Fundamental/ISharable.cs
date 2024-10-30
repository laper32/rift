// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Fundamental;

/// <summary>
/// 所有接口都必须继承该接口.
/// </summary>
public interface ISharable
{
    /// <summary>
    /// 接口名
    /// </summary>
    string InterfaceName { get; }

    /// <summary>
    /// 接口版本
    /// </summary>
    uint InterfaceVersion { get; }
}