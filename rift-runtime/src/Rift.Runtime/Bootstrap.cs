// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manager;
using Rift.Runtime.API.System;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Interop;
using Rift.Runtime.Manager;
using Rift.Runtime.System;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.SystemConsole.Themes;

// Rift Runtime is running with host, so we need to make sure .NET Runtime does 
// not make any unexpected marshalling work.
[assembly: DisableRuntimeMarshalling]

namespace Rift.Runtime;

public static class Bootstrap
{
    [UnmanagedCallersOnly]
    private static bool Init(nint natives)
    {
        InteropService.Init(natives);
        return InitImpl();
    }

    [UnmanagedCallersOnly]
    private static void Shutdown()
    {
        ShutdownImpl();
    }

    private static bool InitImpl()
    {
        var services = new ServiceCollection();
        ConfigureLogging(services);
        ConfigureServices(services);

        var provider = services.BuildServiceProvider(new ServiceProviderOptions
        {
            ValidateOnBuild = true,
            ValidateScopes = true
        });

        ActivateServices(provider);
        
        // ReSharper disable once ConvertIfStatementToReturnStatement
        if (!Boot())
        {
            return false;
        }

        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        workspaceManager.SetRootPath(Path.Combine(Environment.CurrentDirectory, Definitions.ManifestIdentifier));

        try
        {
            workspaceManager.LoadWorkspace();
        }
        catch (Exception e)
        {
            Console.WriteLine($"Error when loading workspace: {e.Message}");
        }

        return true;
    }

    private static void ShutdownImpl()
    {
        ShutdownManagers();
        ShutdownSystems();
    }

    private static bool Boot()
    {
        if (!InitSystems())
        {
            return false;
        }

        // ReSharper disable once ConvertIfStatementToReturnStatement
        if (!InitManagers())
        {
            return false;
        }

        
        return true;
    }

    private static void ConfigureLogging(IServiceCollection services)
    {
        const string consoleTemplate = "L [{Timestamp:MM/dd HH:mm:ss}] | {Level} | {SourceContext}{Scope} {NewLine}{Message:lj}{NewLine}{Exception}{NewLine}";
        Log.Logger = new LoggerConfiguration()
            .Enrich.FromLogContext()
            .MinimumLevel.Verbose()
            .MinimumLevel.Override("Microsoft", LogEventLevel.Information)
            .MinimumLevel.Override("System.Net.Http", LogEventLevel.Fatal)
            .WriteTo.Logger(lc => lc
                .Filter.ByIncludingOnly(e => e.Level >= LogEventLevel.Information)
                .WriteTo.Console(theme: AnsiConsoleTheme.Code,
                    outputTemplate: consoleTemplate,
                    applyThemeToRedirectedOutput: false))
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
        services.AddSingleton<IScriptSystemInternal, ScriptSystem>();
        services.AddSingleton<IPluginSystemInternal, PluginSystem>();

        services.AddSingleton<IWorkspaceManagerInternal, WorkspaceManager>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<IRuntimeInternal>();
        provider.GetRequiredService<IShareSystemInternal>();
        provider.GetRequiredService<IScriptSystemInternal>();
        provider.GetRequiredService<IPluginSystemInternal>();

        provider.GetRequiredService<IWorkspaceManagerInternal>();
    }

    private static bool InitSystems()
    {
        var shareSystem = (ShareSystem)IShareSystem.Instance;
        if (!shareSystem.Init())
        {
            Console.WriteLine("Failed to init ShareSystem.");
            return false;
        }

        var scriptSystem = (ScriptSystem)IScriptSystem.Instance;
        // ReSharper disable once InvertIf
        if (!scriptSystem.Init())
        {
            Console.WriteLine("Failed to initialize ScriptSystem.");
            return false;
        }

        return true;
    }

    private static void ShutdownSystems()
    {
        var scriptSystem = (ScriptSystem)IScriptSystem.Instance;
        scriptSystem.Shutdown();

        var shareSystem = (ShareSystem)IShareSystem.Instance;
        shareSystem.Shutdown();
    }

    private static bool InitManagers()
    {
        var workspaceManager = (WorkspaceManager)IWorkspaceManager.Instance;

        // ReSharper disable once ConvertIfStatementToReturnStatement
        if (!workspaceManager.Init())
        {
            return false;
        }

        return true;
    }

    private static void ShutdownManagers()
    {
        var workspaceManager = (WorkspaceManager)IWorkspaceManager.Instance;
        workspaceManager.Shutdown();
    }
}
