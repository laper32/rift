// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Plugins;

/// <summary>
///     插件入口
/// </summary>
public interface IPlugin
{
    /// <summary>
    ///     当插件准备加载时调用
    /// </summary>
    /// <returns> True if success, false otherwise </returns>
    bool OnLoad();

    /// <summary>
    ///     当所有插件全部加载后调用
    /// </summary>
    void OnAllLoaded();

    /// <summary>
    ///     当插件卸载时调用
    /// </summary>
    void OnUnload();
}

/// <summary>
///     插件基类 <br />
///     所有插件入口必须继承该接口。
/// </summary>
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