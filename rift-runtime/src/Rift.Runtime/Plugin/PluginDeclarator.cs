namespace Rift.Runtime.Plugin;

internal record PluginDeclarator(string Name, string Version);

/// <summary>
/// 获取可能的插件。 <br/>
/// 只在列出所有可能的插件的时候使用。
/// </summary>
/// <param name="Name"></param>
/// <param name="version"></param>
internal record PluginEnumerateDeclarator(string Name, Version version);