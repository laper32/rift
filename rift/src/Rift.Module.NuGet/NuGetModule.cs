using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Modules.Abstractions;
using Rift.Runtime.Modules.Attributes;

//[assembly: ModuleShared]

namespace Rift.Module.NuGet;

internal class NuGetModule : RiftModule
{
    public override bool OnLoad()
    {
        var services = new ServiceCollection();

        var provider = services.BuildServiceProvider();
        //InterfaceManager.AddInterface(provider.GetRequiredService<ExampleService>(), this);

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