using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Abstractions;
using Rift.Generate.Fundamental;
using Rift.Generate.Services;
using Rift.Runtime.Abstractions.Plugin;
using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        TaskManager.RegisterTask("rift.basic", config =>
        {
            config.SetIsCommand(true);
            
        });

        TaskManager.RegisterTask("rift.basic.second", config =>
        {
            config.SetIsCommand(true);
            config.SetDescription("Basic->Second");
            config.AddAction(() =>
            {
                Console.WriteLine("rift.basic.second invoked.");
            });
        });
        var task = TaskManager.RegisterTask("rift.generate", config =>
        {
            config
                .SetIsCommand(true)
                .SetDeferException(true)
                .SetErrorHandler((exception, context) =>
                {
                    Console.WriteLine($"ErrorHandler, {exception.GetType()}, message: {exception.Message}");
                    
                    return Task.CompletedTask;
                })
                .AddAction(() =>
                {
                    GenerateService.Instance.Invoke();
                })
                ;
        });
        var services = new ServiceCollection();
        services.AddSingleton<InterfaceBridge>();
        services.AddSingleton<IGenerateService, GenerateService>();
        services.BuildServiceProvider();

        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        Console.WriteLine("Rift.Generate.OnAllLoaded Ok.");
        base.OnAllLoaded();
    }

    public override void OnUnload()
    {
        Console.WriteLine("Rift.Generate.OnUnload OK.");
        base.OnUnload();
    }
}