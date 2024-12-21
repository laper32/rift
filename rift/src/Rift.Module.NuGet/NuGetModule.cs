using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Interfaces;
using Rift.Runtime.Modules.Abstractions;

namespace Rift.Module.NuGet;

public interface IExampleService : IInterface
{

}

internal class ExampleService : IExampleService
{
    public uint InterfaceVersion => 1;
}

internal class NuGetModule : RiftModule
{
    public override bool OnLoad()
    {
        var services = new ServiceCollection();

        services.AddSingleton<ExampleService>();
        var provider = services.BuildServiceProvider();
        InterfaceManager.AddInterface(provider.GetRequiredService<ExampleService>(), this);

        Console.WriteLine("NuGetModule.OnLoad");
        return true;
    }

    public override void OnAllLoaded()
    {
        Console.WriteLine("NuGetModule.OnAllLoaded");
    }

    public override void OnUnload()
    {
        Console.WriteLine("NuGetModule.OnUnload");
    }
}