using Rift.Runtime.Plugins.Abstractions;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class GeneratePlugin : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Plugin.Generate.OnLoad");
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        Console.WriteLine("Rift.Plugin.Generate.OnAllLoaded");
        base.OnAllLoaded();
    }

    public override void OnUnload()
    {
        Console.WriteLine("Rift.Plugin.Generate.OnUnload");
        base.OnUnload();
    }
}