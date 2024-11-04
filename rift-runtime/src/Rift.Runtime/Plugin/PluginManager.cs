// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Plugin;

internal interface IPluginManagerInternal : IPluginManager, IInitializable
{
    void LoadPlugins();

    bool AddDependencyForPlugin(IPackageImportDeclarator declarator);
    bool AddDependencyForPlugin(IEnumerable<IPackageImportDeclarator> declarators);
    bool AddMetadataForPlugin(string key, object value);
}

internal class PluginManager : IPluginManagerInternal
{
    private readonly PluginIdentities _identities = new();
    public PluginManager()
    {
        IPluginManager.Instance = this;
    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {

    }

    public void LoadPlugins()
    {
        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        var declarators = workspaceManager.CollectPluginsForLoad();
        foreach (var declarator in declarators)
        {
            _identities.FindPlugin(declarator);
        }
        //var result = JsonSerializer.Serialize(declarators, new JsonSerializerOptions
        //{
        //    WriteIndented = true
        //});
        //Console.WriteLine(result);
    }


    ///// <summary>
    ///// 获取插件入口dll <br/>
    /////     <remarks>
    /////         需要特别处理文件的大小写问题，这个函数不负责这个！
    /////     </remarks>
    ///// </summary>
    ///// <param name="pluginName">插件名</param>
    ///// <param name="libraryPath">存放插件入口的文件夹路径</param>
    ///// <returns></returns>
    //private static string GetEntryDll(string pluginName, string libraryPath)
    //{
    //    var conf = Directory.GetFiles(libraryPath, $"*.{PluginEntryToken}");

    //    // ReSharper disable CommentTypo
    //    var entryDll = conf.Length != 1                      // 首先看.deps.json的数量
    //        ? Path.Combine(libraryPath, $"{pluginName}.dll") // 如果没有, 看和文件夹同名的.dll
    //        : conf[0].Replace($".{PluginEntryToken}", ".dll");         // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
    //    // ReSharper restore CommentTypo
    //    return File.Exists(entryDll) ? entryDll : string.Empty;
    //}

    public bool AddDependencyForPlugin(IPackageImportDeclarator declarator)
    {
        return _identities.AddDependencyForPlugin(declarator);
    }

    public bool AddDependencyForPlugin(IEnumerable<IPackageImportDeclarator> declarators)
    {
        return _identities.AddDependencyForPlugin(declarators);
    }

    public bool AddMetadataForPlugin(string key, object value)
    {
        return _identities.AddMetadataForPlugin(key, value);
    }
}