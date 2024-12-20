// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Interfaces;

/// <summary>
///     一般只在接口未找到时用，如果接口未找到则抛出此异常.
/// </summary>
/// <param name="message"> </param>
public class InterfaceNotFoundException(string message = "") : Exception(message);