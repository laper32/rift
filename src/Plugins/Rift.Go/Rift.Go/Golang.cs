using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Abstractions;
using Rift.Go.Fundamental;
using Rift.Go.Services;
using Rift.Runtime.Abstractions.Plugin;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
public class Golang : RiftPlugin
{
    private IGenerateService _generateService = null!;
    public override bool OnLoad()
    {
        _generateService = ShareSystem.GetRequiredInterface<IGenerateService>(1);


        var services = new ServiceCollection();
        services.AddSingleton(this);
        services.AddSingleton<InterfaceBridge>();
        services.AddSingleton<GolangGenerateService>();
        var provider = services.BuildServiceProvider();

        provider.GetRequiredService<GolangGenerateService>();

        ScriptManager.AddLibrary(["Rift.Go"]);
        ScriptManager.AddNamespace("Rift.Go");
        ScriptManager.AddNamespace("Rift.Go.Scripting");

        _generateService.Generate += GolangGenerateService.Instance.PerformGolangGenerate;

        return base.OnLoad();
    }

    public override void OnUnload()
    {
        _generateService.Generate -= GolangGenerateService.Instance.PerformGolangGenerate;
    }
}