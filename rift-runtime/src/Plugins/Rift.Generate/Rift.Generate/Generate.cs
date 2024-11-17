using Rift.Runtime.API.Plugin;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
public class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Generate.OnLoad OK.");
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