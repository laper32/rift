﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


namespace Rift.Runtime;

//public static class Bootstrap
//{
//    public static bool Init()
//    {
//        return InitImpl();
//    }

//    public static void Shutdown()
//    {
//        ShutdownImpl();
//    }

//    public static void Load()
//    {
//        // TODO: 要配合命令行的行为。
//        // TODO: 这里的意思是：如果有subcommand，除非特定的命令，否则走加载workspace流程。
//        WorkspaceManagerInternal.Instance.SetRootPath(Path.Combine(Environment.CurrentDirectory, Definitions.ManifestIdentifier));

//        try
//        {
//            WorkspaceManagerInternal.Instance.LoadWorkspace();
//        }
//        catch (Exception e)
//        {
//            Console.WriteLine($"Error when loading workspace: {e.Message}");
//        }

//        PluginManagerInternal.Instance.NotifyLoadPlugins();

//        var args = Environment.GetCommandLineArgs();
//        Console.WriteLine($"Args: {string.Join(", ", args)}");
//    }

//    private static bool InitImpl()
//    {
//        var services = new ServiceCollection();
//        ConfigureLogging(services);
//        ConfigureServices(services);

//        var provider = services.BuildServiceProvider(new ServiceProviderOptions
//        {
//            ValidateOnBuild = true,
//            ValidateScopes = true
//        });

//        ActivateServices(provider);
        
//        return Boot();
//    }

//    private static void ShutdownImpl()
//    {
//        ShutdownComponents();
//    }

//    private static bool Boot()
//    {
//        try
//        {
//            InitComponents();
//            return true;
//        }
//        catch (Exception e)
//        {
//            Console.WriteLine(e);
//            return false;
//        }
//    }

//    private static void ConfigureLogging(IServiceCollection services)
//    {
//        const string consoleTemplate = "L [{Timestamp:MM/dd HH:mm:ss}] | {Level} | {SourceContext}{Scope} {NewLine}{Message:lj}{NewLine}{Exception}{NewLine}";
//        Log.Logger = new LoggerConfiguration()
//            .Enrich.FromLogContext()
//            .MinimumLevel.Verbose()
//            .MinimumLevel.Override("Microsoft", LogEventLevel.Information)
//            .MinimumLevel.Override("System.Net.Http", LogEventLevel.Fatal)
//            .WriteTo.Logger(lc => lc
//                .Filter.ByIncludingOnly(e => e.Level >= LogEventLevel.Information)
//                .WriteTo.Console(theme: AnsiConsoleTheme.Code,
//                    outputTemplate: consoleTemplate,
//                    applyThemeToRedirectedOutput: false))
//            .CreateLogger();
//        services.AddLogging(logging =>
//        {
//            logging.ClearProviders();
//            logging.AddSerilog();
//            logging.SetMinimumLevel(LogLevel.Information);
//        });
//    }

//    private static void ConfigureServices(IServiceCollection services)
//    {
//        services.AddSingleton<InterfaceBridge>();
//        services.AddSingleton<IRuntimeInternal, Fundamental.Runtime>();
//        services.AddSingleton<IShareSystemInternal, ShareSystem>();
//        services.AddSingleton<ScriptManagerInternal>();
//        services.AddSingleton<PluginManagerInternal>();
//        services.AddSingleton<CommandManagerInternal>();
//        services.AddSingleton<WorkspaceManagerInternal>();
//        services.AddSingleton<TaskManagerInternal>();
//    }

//    private static void ActivateServices(IServiceProvider provider)
//    {
//        provider.GetRequiredService<IRuntimeInternal>();
//        provider.GetRequiredService<IShareSystemInternal>();
//        provider.GetRequiredService<ScriptManagerInternal>();
//        provider.GetRequiredService<PluginManagerInternal>();
//        provider.GetRequiredService<CommandManagerInternal>();
//        provider.GetRequiredService<WorkspaceManagerInternal>();
//        provider.GetRequiredService<TaskManagerInternal>();
//    }

//    private static void InitComponents()
//    {
//        if (!ShareSystemInternal.Instance.Init())
//        {
//            throw new InvalidOperationException("Failed to init ShareSystem.");
//        }

//        //if (!ScriptManagerInternal.Instance.Init())
//        //{
//        //    throw new InvalidOperationException("Failed to init ScriptManager.");
//        //}

//        //if (!PluginManagerInternal.Instance.Init())
//        //{
//        //    throw new InvalidOperationException("Failed to init PluginManager.");
//        //}

//        //if (!WorkspaceManagerInternal.Instance.Init())
//        //{
//        //    throw new InvalidOperationException("Failed to init WorkspaceManager.");
//        //}

//        //if (!TaskManagerInternal.Instance.Init())
//        //{
//        //    throw new InvalidOperationException("Failed to init TaskManager.");
//        //}

//        //if (!CommandManagerInternal.Instance.Init())
//        //{
//        //    throw new InvalidOperationException("Failed to init CommandManager.");
//        //}

//    }

//    private static void ShutdownComponents()
//    {
//        //CommandManagerInternal.Instance.Shutdown();
//        //TaskManagerInternal.Instance.Shutdown();
//        //WorkspaceManagerInternal.Instance.Shutdown();
//        //PluginManagerInternal.Instance.Shutdown();
//        //ScriptManagerInternal.Instance.Shutdown();
//        ShareSystemInternal.Instance.Shutdown();
//    }
    

//}
