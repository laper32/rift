// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.System;

namespace Rift.Runtime.System;

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