// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Attributes;

/// <summary>
/// 插件元数据
/// </summary>
[AttributeUsage(AttributeTargets.Assembly)]
public class PluginAttribute : Attribute
{
    /// <summary>
    /// 插件名
    /// </summary>
    public required string Name { get; set; }

    /// <summary>
    /// 插件作者
    /// </summary>
    public required string Author { get; set; }

    /// <summary>
    /// 所相关的网站
    /// </summary>
    public string Url { get; set; } = string.Empty;

    public override string ToString()
    {
        return $"{GetType().Name}(" +
               $"Name = \"{Name}\", " +
               $"Author = \"{Author}\", " +
               $"Url = \"{Url}\"" +
               $")";
    }
}

[AttributeUsage(AttributeTargets.Assembly)]
public class PluginShared : Attribute;