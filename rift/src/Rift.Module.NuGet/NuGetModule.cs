using Rift.Runtime.Modules.Abstractions;
using Rift.Runtime.Modules.Attributes;

[assembly:ModuleShared]

namespace Rift.Module.NuGet;

internal class NuGetModule : RiftModule
{
    public override bool OnLoad()
    {
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        base.OnAllLoaded();
    }

    public override void OnUnload()
    {
        base.OnUnload();
    }
}