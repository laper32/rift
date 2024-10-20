// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Abstractions;

namespace Rift.Runtime.API.System;

// ReSharper disable UnusedMember.Global

public interface IPluginSystem
{
    /// <summary>
    /// 将插件的状态直接设置为失败, 此时插件将会被强制卸载, 直到下一次重新加载.
    /// </summary>
    /// <param name="instance">插件实例</param>
    /// <param name="reason">失败原因</param>
    void SetFailState(RiftPlugin instance, Exception reason);

    IPluginRuntimeInfo? GetPluginRuntimeInfo(RiftPlugin instance);
    IEnumerable<IPluginRuntimeInfo> GetAllPluginRuntimeInfo();
}
