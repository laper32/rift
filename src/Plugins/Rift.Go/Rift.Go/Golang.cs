using Rift.Generate.Abstractions;
using Rift.Go.API;
using Rift.Runtime.Abstractions.Plugin;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
public class Golang : RiftPlugin
{
    private IGenerateService _generateService = null!;
    public override bool OnLoad()
    {
        
        return base.OnLoad();
    }

    public override void OnUnload()
    {
        base.OnUnload();
    }

    public override void OnAllLoaded()
    {
        base.OnAllLoaded();
    }

    private void Call()
    {
        Console.WriteLine("Invocation from Rift.Go");
    }
}