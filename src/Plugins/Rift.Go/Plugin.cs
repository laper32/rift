using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Go.Generate;
using Rift.Go.Workspace;
using Rift.Runtime.Interfaces;
using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;
using Rift.Runtime.Workspace;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
internal class Plugin : RiftPlugin
{
    private         IGenerateService    _generateService = null!;
    internal static GolangConfiguration Configuration { get; set; } = null!;
    private         IServiceProvider    _provider = null!;
    private static  Plugin              _instance = null!;
    public Plugin()
    {
        Configuration = new GolangConfiguration();
        _instance     = this;
    }

    public override bool OnLoad()
    {
        var services = new ServiceCollection();
        services.AddSingleton(this);
        services.AddSingleton<GolangWorkspaceService>();
        services.AddSingleton<GolangGenerateService>();

        _provider = services.BuildServiceProvider();
        _provider.GetRequiredService<GolangWorkspaceService>();
        _provider.GetRequiredService<GolangGenerateService>();

        WorkspaceManager.AddingReference  += GolangWorkspaceService.OnAddingReference;
        WorkspaceManager.ConfigurePackage += GolangWorkspaceService.OnConfigurePackage;

        ScriptManager.AddLibrary("Rift.Go");
        ScriptManager.AddNamespace(["Rift.Go.Scripting", "Rift.Go.Workspace"]);
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        _generateService          =  InterfaceManager.GetRequiredInterface<IGenerateService>(1);
        _generateService.Generate += GolangGenerateService.PerformGolangGenerate;
    }

    public override void OnUnload()
    {
        _generateService.Generate         -= GolangGenerateService.PerformGolangGenerate;

        WorkspaceManager.ConfigurePackage -= GolangWorkspaceService.OnConfigurePackage;
        WorkspaceManager.AddingReference  -= GolangWorkspaceService.OnAddingReference;

        ScriptManager.RemoveNamespace(["Rift.Go.Scripting", "Rift.Go.Workspace"]);
        ScriptManager.RemoveLibrary("Rift.Go");
    }

    internal static void HasSpecifiedServices()
    {
        var workspaceService = _instance._provider.GetService<GolangWorkspaceService>() is null;
        Console.WriteLine($"GolangWorkspaceService == null: {workspaceService}");
    }
}