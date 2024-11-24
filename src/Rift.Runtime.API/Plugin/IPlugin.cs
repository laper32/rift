// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;

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
    public record PluginInterfaceBridge(
        IRuntime          Runtime,
        IShareSystem      ShareSystem,
        IPluginManager    PluginManager,
        IScriptManager    ScriptManager,
        IWorkspaceManager WorkspaceManager
    );

    private readonly PluginInterfaceBridge _bridge = null!;
    public           Guid                  UniqueId { get; } = Guid.NewGuid();

    public IRuntime          Runtime          => _bridge.Runtime;
    public IShareSystem      ShareSystem      => _bridge.ShareSystem;
    public IPluginManager    PluginManager    => _bridge.PluginManager;
    public IScriptManager    ScriptManager    => _bridge.ScriptManager;
    public IWorkspaceManager WorkspaceManager => _bridge.WorkspaceManager;

    public virtual bool OnLoad() => true;

    public virtual void OnAllLoaded()
    {
    }

    public virtual void OnUnload()
    {
    }
}
