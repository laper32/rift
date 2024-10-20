// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Interop;
using Rift.Runtime.System;


[assembly: DisableRuntimeMarshalling]
namespace Rift.Runtime;

public static class Bootstrap
{
    [UnmanagedCallersOnly]
    private static int Init(nint natives)
    {
        Console.WriteLine("Bootstrap.Init");
        InteropService.Init(natives);
        return Initialize();
    }

    [UnmanagedCallersOnly]
    private static void Shutdown()
    {

    }

    private static int Initialize()
    {
        var services = new ServiceCollection();
        services.AddSingleton<InterfaceBridge>();
        ConfigureLogging(services);
        ConfigureServices(services);

        var provider = services.BuildServiceProvider(new ServiceProviderOptions
        {
            ValidateOnBuild = true, ValidateScopes = true
        });

        ActivateServices(provider);

        return !Boot(provider) ? 1 : 0;
    }

    private static bool Boot(IServiceProvider provider)
    {
        if (!InitSystems(provider))
        {
            return false;
        }

        return true;
    }

    private static void ConfigureLogging(IServiceCollection _)
    {

    }

    private static void ConfigureServices(IServiceCollection services)
    {
        services.AddSingleton<IShareSystemInternal, ShareSystem>();
        services.AddSingleton<IPluginSystemInternal, PluginSystem>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<IShareSystemInternal>();
        provider.GetRequiredService<IPluginSystemInternal>();
    }

    private static bool InitSystems(IServiceProvider provider)
    {
        var shareSystem = provider.GetRequiredService<IShareSystemInternal>();
        if (!shareSystem.Init())
        {
            Console.WriteLine("Failed to startup ShareSystem");
            return false;
        }

        var pluginSystem = provider.GetRequiredService<IPluginSystemInternal>();

        // ReSharper disable once InvertIf
        if (!pluginSystem.Init())
        {
            Console.WriteLine("Failed to startup PluginSystem");
            return false;
        }

        // ..

        return true;
    }
}
