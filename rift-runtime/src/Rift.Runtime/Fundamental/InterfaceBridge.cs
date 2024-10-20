// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.API.System;
using Rift.Runtime.System;

namespace Rift.Runtime.Fundamental;


internal class InterfaceBridge(IServiceProvider provider)
{
    public IShareSystem ShareSystem => provider.GetRequiredService<IShareSystemInternal>();
    public IPluginSystem PluginSystem => provider.GetRequiredService<IPluginSystemInternal>();
}