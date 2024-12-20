using Rift.Runtime.Modules.Abstractions;

namespace Rift.Runtime.Modules.Fundamental;

internal class ModuleInstance
{
    private RiftModule? _instance;

    public bool Init()
    {
        return true;
    }

    public void Load()
    {
        _instance?.OnLoad();
    }

    public void Unload()
    {
        _instance?.OnUnload();
    }

    public void PostLoad()
    {
        _instance?.OnAllLoaded();
    }
}