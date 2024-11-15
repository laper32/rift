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
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Task;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Interop;
using Rift.Runtime.Plugin;
using Rift.Runtime.Scripting;
using Rift.Runtime.Task;
using Rift.Runtime.Workspace;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.SystemConsole.Themes;

// Rift Runtime is running with host, so we need to make sure .NET Runtime does 
// not make any unexpected marshalling work.
[assembly: DisableRuntimeMarshalling]

namespace Rift.Runtime;

internal static class Bootstrap
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

        var pluginManager = (IPluginManagerInternal) IPluginManager.Instance;
        pluginManager.NotifyLoadPlugins();

        return true;
    }

    private static void ShutdownImpl()
    {
        ShutdownComponents();
    }

    private static bool Boot()
    {
        try
        {
            InitComponents();
            return true;
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            return false;
        }
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
        services.AddSingleton<IScriptManagerInternal, ScriptManager>();
        services.AddSingleton<IPluginManagerInternal, PluginManager>();

        services.AddSingleton<IWorkspaceManagerInternal, WorkspaceManager>();
        services.AddSingleton<ITaskManagerInternal, TaskManager>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<IRuntimeInternal>();
        provider.GetRequiredService<IShareSystemInternal>();
        provider.GetRequiredService<IScriptManagerInternal>();
        provider.GetRequiredService<IPluginManagerInternal>();

        provider.GetRequiredService<IWorkspaceManagerInternal>();
        provider.GetRequiredService<ITaskManagerInternal>();
    }

    private static void InitComponents()
    {
        var shareSystem = (ShareSystem)IShareSystem.Instance;
        if (!shareSystem.Init())
        {
            throw new InvalidOperationException("Shutdown to init ShareSystem.");
        }

        var scriptManager = (ScriptManager)IScriptManager.Instance;
        if (!scriptManager.Init())
        {
            throw new InvalidOperationException("Shutdown to init ScriptManager.");
        }

        var pluginManager = (PluginManager) IPluginManager.Instance;
        if (!pluginManager.Init())
        {
            throw new InvalidOperationException("Shutdown to init PluginManager.");
        }

        var workspaceManager = (WorkspaceManager)IWorkspaceManager.Instance;
        if (!workspaceManager.Init())
        {
            throw new InvalidOperationException("Shutdown to init WorkspaceManager.");
        }

        var taskManager = (TaskManager)ITaskManager.Instance;
        if (!taskManager.Init())
        {
            throw new InvalidOperationException("Shutdown to init TaskManager.");
        }
    }

    private static void ShutdownComponents()
    {
        var taskManager = (TaskManager) ITaskManager.Instance;
        taskManager.Shutdown();

        var workspaceManager = (WorkspaceManager)IWorkspaceManager.Instance;
        workspaceManager.Shutdown();

        var pluginManager = (PluginManager) IPluginManager.Instance;
        pluginManager.Shutdown();

        var scriptManager = (ScriptManager) IScriptManager.Instance;
        scriptManager.Shutdown();

        var shareSystem = (ShareSystem)IShareSystem.Instance;
        shareSystem.Shutdown();
    }
}
