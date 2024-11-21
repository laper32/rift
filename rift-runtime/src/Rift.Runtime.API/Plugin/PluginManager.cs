// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Plugin;

public abstract class PluginManager
{
    public static PluginManager Instance { get; protected set; } = null!;

    protected PluginManager()
    {
        Instance = this;
    }
}