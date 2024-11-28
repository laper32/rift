using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Abstractions.Plugin;
using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Generate;

public interface ISampleService
{
    void Call();
}

internal class SampleService : ISampleService
{
    public void Call()
    {
        throw new AccessViolationException("This is completely on purpose");
    }
}

// ReSharper disable once UnusedMember.Global
internal class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        var task = TaskManager.RegisterTask("generate", config =>
        {
            config
                .SetIsCommand(true)
                .SetDeferException(true)
                .SetErrorHandler((exception, context) =>
                {
                    Console.WriteLine("ErrorHandler");
                    return Task.CompletedTask;
                })
                .AddAction(() =>
                {
                    _sampleService.Call();
                })
                ;
        });
        var services = new ServiceCollection();
        services.AddSingleton<ISampleService, SampleService>();
        var provider = services.BuildServiceProvider();
        _sampleService = provider.GetRequiredService<ISampleService>();

        Console.WriteLine(task);

        Console.WriteLine("Rift.Generate.OnLoad OK");
        TaskManager.RunTask("generate");

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

    private ISampleService _sampleService = null!;
}