using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Runtime.Interfaces.Managers;
using Rift.Runtime.IO;
using Rift.Runtime.Plugins.Abstractions;
using Rift.Runtime.Plugins.Annotations;
using Rift.Runtime.Tasks.Managers;

[assembly: PluginShared]

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class GeneratePlugin : RiftPlugin
{
    public override bool OnLoad()
    {
        var services = new ServiceCollection();
        services.AddSingleton<IGenerateService, GenerateService>();
        var provider = services.BuildServiceProvider();
        InterfaceManager.AddInterface(provider.GetRequiredService<IGenerateService>(), this);

        TaskManager.RegisterTask("rift.generate", cfg =>
        {
            cfg.SetIsCommand(true);
            cfg.SetDeferException(true);
            cfg.SetErrorHandler((exception, context) =>
            {
                Tty.Error(exception);

                return Task.CompletedTask;
            });

            cfg.AddAction(_ =>
            {
                GenerateService.Execute();
            });
        });

        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {

    }

    public override void OnUnload()
    {

    }
}