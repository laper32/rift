
using Rift.Runtime.Plugin;
using Rift.Runtime.Tasks;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Generate initialized");
        //var services = new ServiceCollection();
        //services.AddSingleton(this);
        //services.AddSingleton<InterfaceBridge>();
        //services.AddSingleton<IGenerateService, GenerateService>();

        //var provider = services.BuildServiceProvider();
        //provider.GetRequiredService<IGenerateService>();

        TaskManager.Instance.RegisterTask("rift.generate", config =>
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
                    Console.WriteLine("12121");
                });
        });

        return base.OnLoad();
    }
}