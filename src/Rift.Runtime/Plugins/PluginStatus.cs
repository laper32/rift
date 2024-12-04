﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================


namespace Rift.Runtime.Plugins;

/// <summary>
/// 插件状态
/// </summary>
public enum PluginStatus
{
    None = 0,
    /// <summary>
    /// 已加载
    /// </summary>
    Checked,
    /// <summary>
    /// 运行中
    /// </summary>
    Running,
    /// <summary>
    /// 运行时发生错误
    /// </summary>
    Error,
    /// <summary>
    /// 发生错误无法运行
    /// </summary>
    Failed,
}