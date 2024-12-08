using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Go.Fundamental;
using Rift.Go.Generate;
using Rift.Go.Workspace;
using Rift.Runtime.Interfaces;
using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
internal class Plugin : RiftPlugin
{
    private static Plugin           _instance        = null!;
    private        IGenerateService _generateService = null!;
    private        IServiceProvider _provider        = null!;

    public Plugin()
    {
        _instance     = this;
    }

    public override bool OnLoad()
    {
        var services = new ServiceCollection();
        services.AddSingleton(this);
        services.AddSingleton<GolangEnvironment>();
        services.AddSingleton<GolangWorkspaceService>();
        services.AddSingleton<GolangGenerateService>();

        _provider = services.BuildServiceProvider();
        _provider.GetRequiredService<GolangEnvironment>();
        _provider.GetRequiredService<GolangWorkspaceService>();
        _provider.GetRequiredService<GolangGenerateService>();

        ScriptManager.AddLibrary("Rift.Go");
        ScriptManager.AddNamespace(["Rift.Go.Scripting", "Rift.Go.Workspace"]);
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        _generateService          =  InterfaceManager.GetRequiredInterface<IGenerateService>(1);
        _generateService.Generate += GolangGenerateService.PerformGolangGenerate;
        //GolangWorkspaceService.DumpGolangPackages();
    }

    public override void OnUnload()
    {
        _generateService.Generate -= GolangGenerateService.PerformGolangGenerate;

        //WorkspaceManager.AddingReference  -= GolangWorkspaceService.OnAddingReference;
        ScriptManager.RemoveNamespace(["Rift.Go.Scripting", "Rift.Go.Workspace"]);
        ScriptManager.RemoveLibrary("Rift.Go");
    }
}