// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.Commands;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Sharing;
using Rift.Runtime.Plugin;
using Rift.Runtime.Scripting;
using Rift.Runtime.Tasks;
using Rift.Runtime.Workspace;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.SystemConsole.Themes;

[assembly: InternalsVisibleTo("Rift.Runtime.Tests", AllInternalsVisible = true)]

namespace Rift.Runtime;


public static class Bootstrap
{
    public static bool Init()
    {
        return InitImpl();
    }

    public static void Shutdown()
    {
        ShutdownImpl();
    }

    public static void Load()
    {
        // TODO: 要配合命令行的行为。
        // TODO: 这里的意思是：如果有subcommand，除非特定的命令，否则走加载workspace流程。
        WorkspaceManager.Instance.SetRootPath(Path.Combine(Environment.CurrentDirectory, Definitions.ManifestIdentifier));

        try
        {
            WorkspaceManager.Instance.LoadWorkspace();
        }
        catch (Exception e)
        {
            Console.WriteLine($"Error when loading workspace: {e.Message}");
        }

        var args = Environment.GetCommandLineArgs();
        Console.WriteLine($"Args: {string.Join(", ", args)}");
        //PluginManager.Instance.DumpPluginIdentities();
    }

    public static void Run(string[] args)
    {
        CommandManager.Instance.ExecuteCommand(args);
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
        services.AddSingleton<InterfaceBridge>();
        services.AddSingleton<IRuntimeInternal, Fundamental.Runtime>();
        services.AddSingleton<IShareSystemInternal, ShareSystem>();
        services.AddSingleton<IScriptManagerInternal, ScriptManager>();
        services.AddSingleton<IPluginManagerInternal, PluginManager>();
        services.AddSingleton<IWorkspaceManagerInternal, WorkspaceManager>();
        services.AddSingleton<ITaskManagerInternal, TaskManager>();
        services.AddSingleton<ICommandManagerInternal, CommandManager>();
    }

    private static void ActivateServices(IServiceProvider provider)
    {
        provider.GetRequiredService<IRuntimeInternal>();
        provider.GetRequiredService<IShareSystemInternal>();
        provider.GetRequiredService<IScriptManagerInternal>();
        provider.GetRequiredService<IPluginManagerInternal>();
        provider.GetRequiredService<IWorkspaceManagerInternal>();
        provider.GetRequiredService<ITaskManagerInternal>();
        provider.GetRequiredService<ICommandManagerInternal>();
    }

    private static void InitComponents()
    {
        if (!ShareSystem.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init ShareSystem.");
        }

        if (!ScriptManager.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init ScriptManager.");
        }

        if (!PluginManager.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init PluginManager.");
        }

        if (!WorkspaceManager.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init WorkspaceManager.");
        }

        if (!TaskManager.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init TaskManager");
        }

        if (!CommandManager.Instance.Init())
        {
            throw new InvalidOperationException("Failed to init C");
        }
    }

    private static void ShutdownComponents()
    {
        CommandManager.Instance.Shutdown();
        TaskManager.Instance.Shutdown();
        WorkspaceManager.Instance.Shutdown();
        PluginManager.Instance.Shutdown();
        ScriptManager.Instance.Shutdown();
        ShareSystem.Instance.Shutdown();
    }


}
