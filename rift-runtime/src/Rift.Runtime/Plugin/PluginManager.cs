// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Reflection;
using System.Text.Json;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Plugin;

internal interface IPluginManagerInternal : IPluginManager, IInitializable
{
    void LoadPlugins();
}

internal class PluginManager : IPluginManagerInternal
{
    private const string PluginDirectoryName = "plugins";
    private const string PluginLibraryName = "lib";       // eg. ~/.rift/plugins/Example/lib/Example.dll
    private const string PluginBinaryName = "bin";       // eg. ~/.rift/plugins/Google.Protobuf/bin/protoc.exe
    private const string PluginEntryToken = "deps.json"; // 插件不可能没依赖的。
    private readonly string _installationPluginPath = Path.Combine(IRuntime.Instance.InstallationPath, PluginDirectoryName);
    private readonly string _userPluginPath = Path.Combine(IRuntime.Instance.UserPath, PluginDirectoryName);
    private readonly List<PluginEnumerateDeclarator> _installationPathEnumeratedPlugins = [];
    private readonly List<PluginEnumerateDeclarator> _userHomeEnumeratedPlugins = [];

    public PluginManager()
    {
        IPluginManager.Instance = this;
    }

    public bool Init()
    {
        // TODO: 之后这里肯定要改，怎么改我还没想好
        EnumerateInstallationPathPlugins();
        EnumerateUserHomePlugins();
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
            GetPluginFromEnumeratedResult(declarator);
        }
        //var result = JsonSerializer.Serialize(declarators, new JsonSerializerOptions
        //{
        //    WriteIndented = true
        //});
        //Console.WriteLine(result);
    }


    /// <summary>
    /// 获取你安装路径里面装的所有插件路径，即：${YourRiftInstallationPath}/plugins。 <br/>
    /// <br/>
    /// 注：<br/>
    /// <remarks>
    ///     1. 这里只看有没有入口dll，有就OK，没有就直接跳过。
    ///     2. 同时会记录插件版本。
    /// </remarks>
    /// </summary>
    private void EnumerateInstallationPathPlugins()
    {
        var pluginsPath = Directory.GetDirectories(_installationPluginPath);
        foreach (var pluginPath in pluginsPath)
        {
            // TODO: Linux环境下这段是不对的，因为会涉及到大小写的问题。
            // TODO: 这里需要处理文件大小写之后再进行剩下的逻辑。

            var name = Path.GetFileName(pluginPath);
            var dllStoragePath = Path.Combine(pluginPath, PluginLibraryName);
            var entryDll = GetEntryDll(name, dllStoragePath);
            var version = AssemblyName.GetAssemblyName(entryDll).Version!;
            _installationPathEnumeratedPlugins.Add(new PluginEnumerateDeclarator(name, version));
        }
    }

    /// <summary>
    /// 获取用户路径里面装的所有插件路径，即：~/.rift/plugins。 <br/>
    /// <br/>
    /// 注：<br/>
    /// <remarks>
    ///     1. 这里只看有没有入口dll，有就OK，没有就直接跳过。
    ///     2. 同时会记录插件版本。
    /// </remarks>
    /// </summary>
    private void EnumerateUserHomePlugins()
    {
        var pluginsPath = Directory.GetDirectories(_userPluginPath);
        foreach (var pluginPath in pluginsPath)
        {
            // TODO: Linux环境下这段是不对的，因为会涉及到大小写的问题。
            // TODO: 这里需要处理文件大小写之后再进行剩下的逻辑。

            var name = Path.GetFileName(pluginPath);
            var dllStoragePath = Path.Combine(pluginPath, PluginLibraryName);
            var entryDll = GetEntryDll(name, dllStoragePath);
            var version = AssemblyName.GetAssemblyName(entryDll).Version!;
            _userHomeEnumeratedPlugins.Add(new PluginEnumerateDeclarator(name, version));
        }
    }

    private void GetPluginFromEnumeratedResult(PluginDeclarator declarator)
    {
        /*
         如何选择插件？
        
        1. 版本号：如果传过来的版本号是latest，那么直接找最新版，否则就是指定的版本。

        2. 怎么找：在不考虑上下文的前提下（如版本号，指定了一定要用某个路径下的插件），
        插件的搜索规则一定是：Rift的安装目录，~/.rift，项目根目录。

        如果指定了版本号，那么：
            如果版本号是latest，那么将会看搜索路径里哪个实例是latest的，然后加载。
        同样的，这样会涉及到插件依赖的版本号，规则同样。
            如果指定了版本号，同样按照默认规则搜，只不过这回哪个有就返回哪个。
         */
    }

    /// <summary>
    /// 获取插件入口dll <br/>
    ///     <remarks>
    ///         需要特别处理文件的大小写问题，这个函数不负责这个！
    ///     </remarks>
    /// </summary>
    /// <param name="pluginName">插件名</param>
    /// <param name="libraryPath">存放插件入口的文件夹路径</param>
    /// <returns></returns>
    private static string GetEntryDll(string pluginName, string libraryPath)
    {
        var conf = Directory.GetFiles(libraryPath, $"*.{PluginEntryToken}");

        // ReSharper disable CommentTypo
        var entryDll = conf.Length != 1                      // 首先看.deps.json的数量
            ? Path.Combine(libraryPath, $"{pluginName}.dll") // 如果没有, 看和文件夹同名的.dll
            : conf[0].Replace($".{PluginEntryToken}", ".dll");         // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
        // ReSharper restore CommentTypo
        return File.Exists(entryDll) ? entryDll : string.Empty;
    }

}