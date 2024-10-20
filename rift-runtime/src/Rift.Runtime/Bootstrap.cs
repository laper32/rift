// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Interop;
using Rift.Runtime.System;
using Serilog;
using Serilog.Events;

// Rift Runtime is running with host, so we need to make sure .NET Runtime does 
// not make any unexpected marshalling work.
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
            ValidateOnBuild = true,
            ValidateScopes = true
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

    private static void ConfigureLogging(IServiceCollection services)
    {
        Log.Logger = new LoggerConfiguration()
            .Enrich.FromLogContext()
            .MinimumLevel.Verbose()
            .MinimumLevel.Override("Microsoft", LogEventLevel.Information)
            .MinimumLevel.Override("System.Net.Http", LogEventLevel.Fatal)
            .CreateLogger();
        services.AddLogging(logging =>
        {
            logging.ClearProviders();
            logging.AddSerilog();
            logging.SetMinimumLevel(LogLevel.Information);
        });
    }

    private static void ConfigureServices(IServiceCollection services)
    {
        services.AddSingleton<IRuntimeInternal, Fundamental.Runtime>();
        services.AddSingleton<IShareSystemInternal, ShareSystem>();
        services.AddSingleton<IPluginSystemInternal, PluginSystem>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<IRuntimeInternal>();
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
