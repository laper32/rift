// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Interfaces;

/// <summary>
///     所有接口都必须继承该接口.
/// </summary>
public interface IInterface
{
    /// <summary>
    ///     接口版本
    /// </summary>
    uint InterfaceVersion { get; }
}