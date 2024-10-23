// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.System;

public interface IScriptSystem
{
    public static IScriptSystem Instance { get; protected set; } = null!;
}