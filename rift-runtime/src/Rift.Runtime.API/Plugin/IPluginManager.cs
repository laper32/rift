// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Plugin;

public interface IPluginManager
{
    public static IPluginManager Instance { get; protected set; } = null!;
}