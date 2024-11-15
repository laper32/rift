// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Plugin;

// TODO: 未来这部分应该是由ModuleSystem来处理会更好？

// ReSharper disable UnusedMemberInSuper.Global

public interface IPlugin
{
    bool OnLoad();

    void OnAllLoaded();

    void OnUnload();
}

public abstract class RiftPlugin : IPlugin
{
    public Guid UniqueId { get; } = Guid.NewGuid();

    public virtual bool OnLoad() => true;

    public virtual void OnAllLoaded()
    {
    }

    public virtual void OnUnload()
    {
    }
}
