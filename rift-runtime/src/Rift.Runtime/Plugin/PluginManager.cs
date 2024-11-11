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
    private          PluginInstanceContext? _sharedContext   = null;
    private const    string                 LibPathName      = "lib";
    private const    string                 PluginEntryToken = "deps.json";

    private readonly List<string> _pendingLoadPluginEntryPath       = [];
    private readonly List<string> _pendingLoadPluginPath            = [];
    private readonly List<string> _pendingPluginSharedAssemblyPaths = [];

    private readonly List<PluginInstance> _instances      = [];
    private readonly List<PluginContext>  _pluginContexts = [];

    private readonly Dictionary<
        string,                        // Shared Assembly Name
        List<PluginSharedAssemblyInfo> // List<SharedAssemblyInfo>
    > _pendingPluginSharedAssemblyInfos = [];

    private readonly List<PluginSharedAssemblyInfo> _sharedAssemblyInfos = [];

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

        //_identities.Dump();

        var pluginIdentities = _identities.GetSortedIdentities();
        AnalyzePluginsEntry(pluginIdentities);
        //RecordSharedAssemblies();
        //RecordSharedAssemblyInfos();
        //MakeSharedAssemblyPriority();
        //LoadSharedContext();
        //LoadPluginContext();
    }

    private void AnalyzePluginsEntry(IEnumerable<PluginIdentity> identities)
    {
        foreach (var identity in identities)
        {
            var libPath = Path.Combine(identity.Location, LibPathName);

            // TODO: 大小写敏感的路径支持
            var entryDll = GetEntryDll(identity.Value.Name, libPath);
            if (string.IsNullOrEmpty(entryDll))
            {
                continue;
            }

            _pendingLoadPluginEntryPath.Add(entryDll);
            _pendingLoadPluginPath.Add(identity.Location);
        }

        Console.WriteLine("_pendingLoadPluginEntryPath...");

    }

    private void RecordSharedAssemblies()
    {
        RecordPluginSharedAssemblies();
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

    private void MakeSharedAssemblyPriority()
    {
        // 排序规则
        // 1. ProductVersion: Major > Minor > Build
        // 2. 上一次的文件修改日期, 以最近的为基准
        // 3. 如果前两个还是不行, 那么就只拿列表中的第一个
        // N.B. 大多数情况下, 版本号只有Major.Minor.Build, 第四个是Revision, FileVersionInfo里管它叫Private, 用法千奇百怪,
        // 但无论如何前三个是我所见到的各种打版本号规则里, 最为通用的.
        //
        // 如果Major.Minor.Build都相同, 且此时我们发现同名的dll有多个, 那么就有如下情况:
        //  1. 真的是一模一样的dll, 发行时为了保险加了多份.
        //  2. 开发阶段: 因为开发阶段往往是下一个版本, 但还没定稿, 这时候沿用原来的版本号也是很正常的.
        // 如果是情况1, 那么一切好说, 重点是情况2: 这时候只能看dll的修改时间来判断, 如果这个dll修改日期越近, 那自然就能说明
        // 这个dll是最新版, 反之, 版本号比较老.
        //
        // 此外, 如果说, Major.Minor.Build一样, 这里设置两个同名dll, 叫A.1和A.2, A.1比A.2要旧, 但他们的版本号一致, 存在一种情况,
        // A.1的修改日期比A.2要新(这种一般是因为有些工具写入文件选择直接创建新文件导致的)
        // 我不认为我们需要处理这种情况, 因为既然有这种情况发生, 说明用户已经知晓了这种行为可能会带来的后果. 而且就我所知的大多数情况下, 
        // 版本号相同而实际执行出现问题, 这种是属于插件作者的问题, 而不是框架的问题.
        // 记住, 我们只能保证提供的是最新版, 但我们不能保证内部的执行细节.
        foreach (var (_, value) in _pendingPluginSharedAssemblyInfos)
        {
            // 这里主要是为了尽量兼顾开发和运行.
            // 开发阶段往往会出现这么个情况: 东西没写完, 版本号不递进, 但是每次都在编译, 每次都有产出.
            // 那么这时候版本号一定是一样的, 但区别在于写入日期.
            // 而为了保证插件的正常运行, 我们自然会选择最近写入的dll
            // 如果是正式上线的话, 你不应该走到第四步, 也就是看写入日期这一步.
            // 如果真走到了这一步, 一般情况下，你不如想想你的整个项目组织是不是出问题了.
            // (非一般情况如：CS2更新，需要紧急修复游戏更新导致的炸服，根本不可能留时间走完CI的全套流程)
            // 此外, 开发阶段应当也是基于版本号的. 因为经常有backport的情况出现, 重点还是为了开发方便就是了.
            var first = value
                .OrderByDescending(x => x.Info.ProductMajorPart)
                .ThenByDescending(x => x.Info.ProductMinorPart)
                .ThenByDescending(x => x.Info.ProductBuildPart)
                .ThenByDescending(x => x.LastWriteDate)
                .First();

            _sharedAssemblyInfos.Add(first);
        }
    }

    private void RecordPluginSharedAssemblies()
    {
        var libPaths = _pendingLoadPluginPath.Select(pluginPath => Path.Combine(pluginPath, LibPathName)).ToList();

        libPaths.ForEach(x =>
        {
            _pendingPluginSharedAssemblyPaths.AddRange(GetPluginSharedAssembliesPath(x));
        });
    }

    private void LoadSharedContext()
    {
        foreach (var info in _sharedAssemblyInfos)
        {
            using var fs = new FileStream(info.Path, FileMode.Open);
            _sharedContext!.LoadFromStream(fs);
        }
    }

    private void LoadPluginContext()
    {
        foreach (var entryPath in _pendingLoadPluginEntryPath)
        {
            _pluginContexts.Add(new PluginContext(entryPath, _sharedContext!));
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