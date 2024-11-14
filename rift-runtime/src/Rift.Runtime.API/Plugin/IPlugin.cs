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


public interface IPluginRuntimeInfo
{
    string Name { get; }
    string Author { get; }
    string Version { get; }
    string Url { get; }
    string Description { get; }
    string Path { get; }
    string EntryPath { get; }
    string Identifier { get; }
    PluginStatus Status { get; }
    Guid UniqueId { get; }
    Exception? Error { get; }
}

public abstract class RiftPlugin : IPlugin
{
    public record PluginInfo(string Name, string Author, string Version, string Url, string Description);

    public PluginInfo MyInfo = null!;

    public record PluginInterfaceBridge(string InstancePath, string RootPath);

    private readonly PluginInterfaceBridge _bridge = null!;

    /// <summary>
    /// 实例路径, 也就是正在运行的这个.dll的路径
    /// </summary>
    public string InstancePath => _bridge.InstancePath;

    /// <summary>
    /// 当前正在运行插件的文件夹路径.
    /// </summary>
    public string MyPath => _bridge.RootPath;

    public Guid UniqueId { get; } = Guid.NewGuid();

    public virtual bool OnLoad() => true;

    public virtual void OnAllLoaded()
    {
    }

    public virtual void OnUnload()
    {
    }
}
