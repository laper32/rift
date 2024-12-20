// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Plugins;

/// <summary>
///     如果你的插件需要共享，则需要对特定的插件Assembly打上此标记。
/// </summary>
[AttributeUsage(AttributeTargets.Assembly)]
public class PluginSharedAttribute : Attribute;