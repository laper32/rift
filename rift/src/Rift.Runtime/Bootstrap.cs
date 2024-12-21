// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.Application;
using Rift.Runtime.Commands.Managers;
using Rift.Runtime.Constants;
using Rift.Runtime.Interfaces.Managers;
using Rift.Runtime.IO;
using Rift.Runtime.Modules.Managers;
using Rift.Runtime.Plugins.Managers;
using Rift.Runtime.Scripts.Managers;
using Rift.Runtime.Tasks.Managers;
using Rift.Runtime.Workspace.Managers;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.SystemConsole.Themes;

[assembly: InternalsVisibleTo("Rift.Runtime.Tests", AllInternalsVisible = true)]
[assembly: InternalsVisibleTo("Rift", AllInternalsVisible = true)]

namespace Rift.Runtime;

internal static class Bootstrap
{
    internal static bool Init()
    {
        return InitImpl();
    }

    internal static void Shutdown()
    {
        ShutdownImpl();
    }

    internal static void Exec(string[] args)
    {
        var rootManifestPath = Path.Combine(Environment.CurrentDirectory, Definitions.ManifestIdentifier);
        if (!File.Exists(rootManifestPath))
        {
            CommandManager.ExecuteCommand(args);
        }
        else
        {
            WorkspaceManager.SetRootPath(rootManifestPath);

            try
            {
                WorkspaceManager.LoadWorkspace();
            }
            catch (Exception e)
            {
                Tty.Error($"{e.Message}");
            }

            CommandManager.ExecuteCommand(args);
        }

        TaskManager.RunTasks();
    }

    private static bool InitImpl()
    {
        var services = new ServiceCollection();
        ConfigureLogging(services);
        ConfigureServices(services);

        var provider = services.BuildServiceProvider(new ServiceProviderOptions
        {
            ValidateOnBuild = true,
            ValidateScopes  = true
        });

        ActivateServices(provider);

        return Boot();
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
        const string consoleTemplate =
            "L [{Timestamp:MM/dd HH:mm:ss}] | {Level} | {SourceContext}{Scope} {NewLine}{Message:lj}{NewLine}{Exception}{NewLine}";
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
        services.AddSingleton<ApplicationHost>();
        services.AddSingleton<ModuleManager>();
        services.AddSingleton<InterfaceManager>();
        services.AddSingleton<ScriptManager>();
        services.AddSingleton<PluginManager>();
        services.AddSingleton<WorkspaceManager>();
        services.AddSingleton<TaskManager>();
        services.AddSingleton<CommandManager>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<ApplicationHost>();
        provider.GetRequiredService<ModuleManager>();
        provider.GetRequiredService<InterfaceManager>();
        provider.GetRequiredService<ScriptManager>();
        provider.GetRequiredService<PluginManager>();
        provider.GetRequiredService<WorkspaceManager>();
        provider.GetRequiredService<TaskManager>();
        provider.GetRequiredService<CommandManager>();
    }

    private static void InitComponents()
    {
        if (!InterfaceManager.Init())
        {
            throw new InvalidOperationException("Failed to init InterfaceManager.");
        }

        if (!ModuleManager.Init())
        {
            throw new InvalidOperationException($"Failed to init {typeof(ModuleManager)}");
        }

        //if (!ScriptManager.Init())
        //{
        //    throw new InvalidOperationException($"Failed to init {nameof(ScriptManager)}.");
        //}

        //if (!PluginManager.Init())
        //{
        //    throw new InvalidOperationException($"Failed to init {nameof(PluginManager)}.");
        //}

        //if (!WorkspaceManager.Init())
        //{
        //    throw new InvalidOperationException($"Failed to init {nameof(WorkspaceManager)}.");
        //}

        //if (!TaskManager.Init())
        //{
        //    throw new InvalidOperationException($"Failed to init {nameof(TaskManager)}.");
        //}

        //if (!CommandManager.Init())
        //{
        //    throw new InvalidOperationException($"Failed to init {nameof(CommandManager)}");
        //}
    }

    private static void ShutdownComponents()
    {
        //CommandManager.Shutdown();
        //TaskManager.Shutdown();
        //WorkspaceManager.Shutdown();
        //PluginManager.Shutdown();
        //ScriptManager.Shutdown();
        ModuleManager.Shutdown();
        InterfaceManager.Shutdown();
    }
}