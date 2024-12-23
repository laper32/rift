// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Plugins.Fundamental;

namespace Rift.Runtime.Plugins.Managers;

public sealed class PluginManager
{
    private static PluginManager _instance = null!;

    private readonly List<PluginIdentity> _pendingLoadPlugins = [];

    private readonly List<PluginInstance> _internalPlugins  = [];
    private readonly List<PluginInstance> _workspacePlugins = [];

    public PluginManager()
    {
        _instance = this;
    }

    internal static bool Init()
    {
        _instance.ActivateInternalPlugins();
        return true;
    }

    internal static void Shutdown()
    {
    }

    #region Preload Plugins

    /// <summary>
    /// 加载内核插件，其包括：<br/>
    /// - 安装路径下的plugins目录 <br />
    /// - 用户.rift路径下的plugins目录 <br/>
    /// </summary>
    internal void ActivateInternalPlugins()
    {

    }

    #endregion

    #region Workspace Plugins

    internal static void ActivateWorkspaceDeclaredPlugins()
    {
    }

    #endregion

    #region Generic Operations

    // ReSharper disable once MemberCanBeMadeStatic.Local
    private string GetEntryDll(string pluginPath)
    {
        // ReSharper disable StringLiteralTypo
        // ReSharper disable CommentTypo
        var conf = Directory.GetFiles(pluginPath, "*.deps.json");
        var entryDll = conf.Length != 1                     // 首先看.deps.json的数量
            ? Path.Combine(pluginPath, $"{pluginPath}.dll") // 如果没有, 看和文件夹同名的.dll
            : conf[0].Replace(".deps.json", ".dll");        // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
        // ReSharper restore CommentTypo
        // ReSharper restore StringLiteralTypo
        return File.Exists(entryDll) ? entryDll : string.Empty;
    }


    #endregion
}