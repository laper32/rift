using Rift.Runtime.Plugins.Abstractions;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
internal class GoPlugin  : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Plugin.Go.OnLoad");
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        Console.WriteLine("Rift.Plugin.Go.OnAllLoaded");
        base.OnAllLoaded();
    }

    public override void OnUnload()
    {
        Console.WriteLine("Rift.Plugin.Go.OnUnload");
        base.OnUnload();
    }
}
