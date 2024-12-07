// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Plugins;

public interface IPlugin
{
    bool OnLoad();
    void OnAllLoaded();
    void OnUnload();
}

public abstract class RiftPlugin : IPlugin
{
    public virtual bool OnLoad()
    {
        return true;
    }

    public virtual void OnAllLoaded()
    {
    }

    public virtual void OnUnload()
    {
    }
}