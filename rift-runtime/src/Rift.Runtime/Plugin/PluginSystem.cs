// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;

namespace Rift.Runtime.Plugin;

internal interface IPluginSystemInternal : IPluginSystem, IInitializable;

internal class PluginSystem : IPluginSystemInternal
{
    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {

    }
}