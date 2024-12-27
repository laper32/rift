using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Go.Application;
using Rift.Go.Generate;
using Rift.Go.Workspace;
using Rift.Runtime.Interfaces.Managers;
using Rift.Runtime.Plugins.Abstractions;
using Rift.Runtime.Plugins.Managers;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
internal class GolangPlugin : RiftPlugin
{
    private GolangGenerateService _golangGenerateService = null!;

    public override bool OnLoad()
    {
        var myInfo = PluginManager.GetPluginRuntimeInfo(this)!;
        var services = new ServiceCollection();
        services.AddSingleton(this);
        services.AddSingleton(myInfo);
        services.AddSingleton<GolangEnvironment>();
        services.AddSingleton<GolangGenerateService>();
        services.AddSingleton<GolangWorkspaceService>();
        var provider = services.BuildServiceProvider();
        _golangGenerateService = provider.GetRequiredService<GolangGenerateService>();

        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        var generateService = InterfaceManager.GetRequiredInterface<IGenerateService>(1);
        generateService.AddListener(this, _golangGenerateService);
    }

    public override void OnUnload()
    {

    }
}
