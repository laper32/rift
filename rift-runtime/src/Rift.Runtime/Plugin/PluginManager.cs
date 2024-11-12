// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using System.Text.Json;
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
    // TODO: 未来的插件系统需要想办法处理没有插件入口的情况。

    private record PluginSharedAssemblyInfo(string Path, FileVersionInfo Info, DateTime LastWriteDate);

    private readonly PluginIdentities       _identities      = new();
    private          PluginInstanceContext? _sharedContext;

    private readonly List<PluginIdentity> _pendingLoadPlugins               = [];
    private readonly List<string>         _pendingPluginSharedAssemblyPaths = [];
    // Key: Shared Assembly Name, Value: SharedAssemblyInfo
    private readonly Dictionary<string, List<PluginSharedAssemblyInfo>> _pendingPluginSharedAssemblyInfos = [];

    public PluginManager()
    {
        IPluginManager.Instance = this;
    }

    public bool Init()
    {
        _sharedContext ??= new PluginInstanceContext();
        return true;
    }

    public void Shutdown()
    {

    }

    /// <summary>
    /// N.B. 插件系统这里和很多地方不一样的是：我们需要支持没有dll的情况（即：这个插件只有二进制文件，或者只有配置文件）<br/>
    /// 所以必须有一个中间层给插件做Identity。
    /// </summary>
    public void LoadPlugins()
    {
        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        var declarators = workspaceManager.CollectPluginsForLoad();
        foreach (var declarator in declarators)
        {
            _identities.Add(declarator);
        }

        var pluginIdentities = _identities.GetSortedIdentities();
        _pendingLoadPlugins.AddRange(pluginIdentities);
        RecordSharedAssemblies();
        RecordSharedAssemblyInfos();
    }

    private void RecordSharedAssemblies()
    {
        foreach (var pendingLoadPlugin in _pendingLoadPlugins)
        {
            _pendingPluginSharedAssemblyPaths.AddRange(GetPluginSharedAssembliesPath(pendingLoadPlugin.LibPath));
        }

        Console.WriteLine("_pendingPluginSharedAssemblyPaths...");
        _pendingPluginSharedAssemblyPaths.ForEach(x =>
        {
            Console.WriteLine($" => {x}");
        });
        Console.WriteLine("...End");
    }

    private void RecordSharedAssemblyInfos()
    {
        foreach (var sharedAssemblyPath in _pendingPluginSharedAssemblyPaths)
        {
            var name = Path.GetFileNameWithoutExtension(sharedAssemblyPath);
            if (!_pendingPluginSharedAssemblyInfos.TryGetValue(name, out var value))
            {
                value                                   = [];
                _pendingPluginSharedAssemblyInfos[name] = value;
            }
       
            value.Add(new PluginSharedAssemblyInfo(
                sharedAssemblyPath,
                FileVersionInfo.GetVersionInfo(sharedAssemblyPath),
                File.GetLastWriteTime(sharedAssemblyPath))
            );
        }
    }


    private IEnumerable<string> GetPluginSharedAssembliesPath(string libraryPath)
    {
        var dlls = Directory.GetFiles(libraryPath, "*.dll");

        // 为了确保插件之间的接口共享, 我们必须得想一个办法让二进制文件在插件内部共享.
        // 我们不能把每个插件的所有依赖全部共享, 如NuGet里面的东西, 因为确实存在一种情况, 不同插件需要不同的nuget包版本
        // 同时这两个版本互相不兼容, 且这时候两个插件互相独立, 互不干扰.
        // 因此, 我们选择打个洞: 如果你插件需要对外的接口共享, 那么你必须给项目打上assembly级别的标签, 也就是`PluginShared`
        // 然后我们通过读PE的方式找到这些带了这个标签的二进制文件, 把他们收集起来, 再根据一定的规则把他们都变成共享context。
        foreach (var dll in dlls)
        {
            using var fs = new FileStream(dll, FileMode.Open);
            using var pe = new PEReader(fs);
            var reader = pe.GetMetadataReader();
            if (!reader.IsAssembly)
            {
                continue;
            }
            // 首先找[assembly:]那一堆attributes.
            var asmDef = reader.GetAssemblyDefinition();
            // ReSharper disable once ForeachCanBeConvertedToQueryUsingAnotherGetEnumerator
            // ReSharper disable once ForeachCanBePartlyConvertedToQueryUsingAnotherGetEnumerator
            foreach (var attribute in asmDef.GetCustomAttributes())
            {
                var attr = reader.GetCustomAttribute(attribute);
                if (attr.Constructor.Kind != HandleKind.MemberReference)
                {
                    continue;
                }

                var memberReference = reader.GetMemberReference((MemberReferenceHandle)attr.Constructor);
                if (memberReference.Parent.Kind != HandleKind.TypeReference)
                {
                    continue;
                }

                var typeReference = reader.GetTypeReference((TypeReferenceHandle)memberReference.Parent);
                if ($"{reader.GetString(typeReference.Namespace)}.{reader.GetString(typeReference.Name)}".Equals(typeof(PluginShared).FullName!))
                {
                    yield return dll;
                }
            }
        }
    }

    /// <summary>
    /// PE相关的内容看<seealso cref="GetPluginSharedAssembliesPath"/>
    /// </summary>
    /// <param name="libraryPath"></param>
    /// <returns></returns>
    private IEnumerable<string> GetScriptSharedAssembliesPath(string libraryPath)
    {
        var dlls = Directory.GetFiles(libraryPath, "*.dll");

        foreach (var dll in dlls)
        {
            using var fs = new FileStream(dll, FileMode.Open);
            using var pe = new PEReader(fs);
            var reader = pe.GetMetadataReader();
            if (!reader.IsAssembly)
            {
                continue;
            }
            // 首先找[assembly:]那一堆attributes.
            var asmDef = reader.GetAssemblyDefinition();
            // ReSharper disable once ForeachCanBeConvertedToQueryUsingAnotherGetEnumerator
            // ReSharper disable once ForeachCanBePartlyConvertedToQueryUsingAnotherGetEnumerator
            foreach (var attribute in asmDef.GetCustomAttributes())
            {
                var attr = reader.GetCustomAttribute(attribute);
                if (attr.Constructor.Kind != HandleKind.MemberReference)
                {
                    continue;
                }

                var memberReference = reader.GetMemberReference((MemberReferenceHandle)attr.Constructor);
                if (memberReference.Parent.Kind != HandleKind.TypeReference)
                {
                    continue;
                }

                var typeReference = reader.GetTypeReference((TypeReferenceHandle)memberReference.Parent);
                if ($"{reader.GetString(typeReference.Namespace)}.{reader.GetString(typeReference.Name)}".Equals(typeof(ScriptShared).FullName!))
                {
                    yield return dll;
                }
            }
        }
    }

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