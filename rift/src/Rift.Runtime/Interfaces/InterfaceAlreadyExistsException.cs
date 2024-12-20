// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Interfaces;

/// <summary>
///     一般只在注册接口时用，如果接口已经存在则抛出此异常.
/// </summary>
/// <param name="message"> </param>
public class InterfaceAlreadyExistsException(string message = "") : Exception(message);