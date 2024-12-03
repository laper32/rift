// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using System.Text.Json;
using System.Text.Json.Serialization;
using Rift.Runtime.Fundamental.Extensions;
using Rift.Runtime.Fundamental.Generic;
using Rift.Runtime.Manifest;
using Rift.Runtime.Scripting;
using Rift.Runtime.Workspace;
using Semver;

namespace Rift.Runtime.Plugins;

internal record PluginSharedAssemblyInfo(string Path, FileVersionInfo Info, DateTime LastWriteDate);


internal class PluginIdentity(
    // 如果你需要类型转换, 用 EitherManifest<RiftManifest<PluginManifest>>
    IMaybePackage package)
{
    public IMaybePackage Value { get; init; } = package;
    public Dictionary<string, object> Dependencies { get; init; } = [];
    public Dictionary<string, object> Metadata { get; init; } = [];
    public string Location => Path.GetFullPath(Directory.GetParent(Value.ManifestPath)!.FullName);
    public string LibPath => Path.Combine(Location, LibPathName);
    public string BinPath => Path.Combine(Location, BinPathName);
    public string EntryPath => GetEntryDll();

    // 一个插件内是不应该出现在一个包里有相同插件但有不同版本的情况的, 这个情况只会在多个插件的时候才会出现.
    // (如: 两个不同的插件A, B, 同时依赖了插件C的不同版本. 但很明显, A或B自身是不可能出现同时引用一个插件的不同版本的情况的.)
    // 如果你使用的插件出现了问题, 你应当考虑这个插件的作者在设计的时候出了问题/你的项目依赖结构有问题
    public Dictionary<string, PluginSharedAssemblyInfo> PluginSharedAssemblyInfos
    {
        get
        {
            var ret       = new Dictionary<string,PluginSharedAssemblyInfo>();
            var sharedAsm = GetPluginSharedAssembliesPath().ToArray();

            if (!sharedAsm.Any())
            {
                return ret;
            }

            foreach (var sharedAssemblyPath in sharedAsm)
            {
                var name = Path.GetFileNameWithoutExtension(sharedAssemblyPath);
                ret.Add(name,
                    new PluginSharedAssemblyInfo(
                        sharedAssemblyPath,
                        FileVersionInfo.GetVersionInfo(sharedAssemblyPath),
                        File.GetLastWriteTime(sharedAssemblyPath)
                    )
                );
            }

            return ret;
        }
    }


    private const string BinPathName = "bin";
    private const string LibPathName = "lib";
    private const string PluginEntryToken = "deps.json";

    private IEnumerable<string> GetPluginSharedAssembliesPath()
    {
        var dlls = Directory.GetFiles(LibPath, "*.dll");

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
    /// 获取插件入口dll <br/>
    ///     <remarks>
    ///         需要特别处理文件的大小写问题，这个函数不负责这个！
    ///     </remarks>
    /// </summary>
    /// <returns></returns>
    private string GetEntryDll()
    {
        var conf = Directory.GetFiles(LibPath, $"*.{PluginEntryToken}");
        // 首先看.deps.json的数量
        var entryDll = conf.Length != 1
            // 如果没有, 看和文件夹同名的.dll
            ? Path.Combine(LibPath, $"{Value.Name}.dll")
            // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
            : conf[0].Replace($".{PluginEntryToken}", ".dll");

        return File.Exists(entryDll) ? entryDll : string.Empty;
    }
}

internal class PluginIdentities()
{
    private const string PluginDirectoryName = "plugins";

    private readonly List<PluginIdentity> _identities = [];

    private PluginIdentity? _currentEvaluatingIdentity;

    private readonly List<string> _pluginSearchPaths =
    [
        Path.Combine(ApplicationHost.Instance.InstallationPath, PluginDirectoryName), // Rift安装路径
        Path.Combine(ApplicationHost.Instance.UserPath, PluginDirectoryName)          // 用户目录
    ];

    private PluginIdentity CreatePluginIdentity(string manifestPath)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EEitherManifest.Rift:
            {
                return manifest switch
                {
                    // 插件是存在一个文件夹里通过版本号作为文件夹区分方式的，所以不能直接打死！
                    EitherManifest<RiftManifest<PluginManifest>> pluginManifest =>
                        new PluginIdentity(
                            new MaybePackage<RiftPackage>(
                                new RiftPackage(pluginManifest.Value, manifestPath)
                            )
                        ),
                    _ => throw new InvalidOperationException("Only supports Rift specific manifests.")
                };
            }
            case EEitherManifest.Virtual:
            case EEitherManifest.Real:
            default:
            {
                throw new InvalidOperationException("Only supports Rift specific manifests.");
            }
        }
    }

    // 插件系统不可能出现套娃情况的，所有插件只会有一层。

    public void AddSearchPath(string path)
    {
        _pluginSearchPaths.Add(path);
    }

    /// <summary>
    /// 根据Descriptor添加Identity <br/>
    /// 注：<br/>
    /// <remarks>
    ///     1. 插件的名字一定是文件夹的名字，且版本是根据文件夹名来做分类。 <br/>
    ///     2. 通过Rift.toml解析插件包。
    /// </remarks>
    /// </summary>
    /// <param name="descriptor"></param>
    public void Add(PluginDescriptor descriptor)
    {
        var uniqueSearchPaths = _pluginSearchPaths.Distinct().ToList();

        /*
         插件的一些规则：
        1. 传入的插件名一定等于其文件夹的名字。
        2. 通过以版本号命名的文件夹做区分。

        插件搜索规则：
        1. 根据提供的搜索路径搜寻插件。
        2. 如果找到了插件，根据提供的搜索路径的先后顺序决定调用哪个插件。

        插件结果筛选：
        1. 如果发现插件的版本等一致，直接用第一个结果
        2. 如果发现版本不一致，排序，然后选出第一个。
        3. 如果标记的是latest，只查最新版。
         */

        // TODO: Windows默认对大小写不敏感，Linux默认大小写敏感。
        // TODO: 我们需要处理Linux环境下同名但大小写不同导致的文件夹不同的问题。
        // TODO: 现版本我们只考虑Windows，不考虑Linux

        var possiblePlugins = new List<PluginIdentity>();
        uniqueSearchPaths.ForEach(x =>
        {
            try
            {
                if (FindFromSearchPath(x, descriptor) is { } identity)
                {
                    possiblePlugins.Add(identity);
                }
            }
            catch (Exception)
            {
                // ignored
            }
        });
        if (possiblePlugins.Count == 0)
        {
            return;
        }
        var possiblePlugin = possiblePlugins.First();

        // 去除重复插件
        // TODO: 未来如果需要同时加载多个版本的话, 应该考虑Package-based Isolated Scope
        // 目前只能是全局
        if (_identities.Exists(x =>
            {
                var nameEquals = x.Value.Name.Equals(possiblePlugin.Value.Name, StringComparison.OrdinalIgnoreCase);
                
                return nameEquals;
            }))
        {
            return;
        }

        _currentEvaluatingIdentity = possiblePlugin;
        RetrievePluginDependencies(possiblePlugin);
        RetrievePluginMetadata(possiblePlugin);
        _identities.Add(possiblePlugin);
        AnalyzeDependencies(possiblePlugin);
    }

    /// <summary>
    /// 从下到上搜索依赖，所以加载顺序应该是反过来的？
    /// </summary>
    /// <returns></returns>
    public List<PluginIdentity> GetSortedIdentities()
    {
        var result = new List<PluginIdentity>(_identities);
        result.Reverse();
        return result;
    }

    public void Dump()
    {
        Console.WriteLine(JsonSerializer.Serialize(_identities, new JsonSerializerOptions
        {
            WriteIndented = true,
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
            Converters =
            {
                new JsonStringEnumConverter(JsonNamingPolicy.CamelCase)
            }
        }));
    }

    private void AnalyzeDependencies(PluginIdentity identity)
    {

        identity.Dependencies.ForEach((_, value) =>
        {
            if (value is not Scripting.Plugin declarator)
            {
                return;
            }

            var name = declarator.Name.Trim();
            if (string.IsNullOrEmpty(name))
            {
                Console.WriteLine("Unknown plugin, skip");
            }

            var version = declarator.Version.Trim();
            if (string.IsNullOrEmpty(version))
            {
                version = "latest";
            }

            Add(new PluginDescriptor(name, version));
        });
    }

    private PluginIdentity? FindFromSearchPath(string path, PluginDescriptor descriptor)
    {
        var pluginPath = Path.Combine(path, descriptor.Name);
        var pluginVersionsDir = Directory.GetDirectories(pluginPath);
        var pluginVersions = new List<SemVersion>();

        foreach (var s in pluginVersionsDir)
        {
            var versionDir = Path.GetFileName(s);
            if (SemVersion.TryParse(versionDir, out var version))
            {
                pluginVersions.Add(version);
            }
        }

        if (pluginVersionsDir.Length <= 0)
        {
            return null;
        }

        var latestVersion = pluginVersions.Max(SemVersion.SortOrderComparer)!;

        if (descriptor.Version.Equals("latest", StringComparison.OrdinalIgnoreCase))
        {
            var latestPluginPath = Path.Combine(pluginPath, latestVersion.ToString());

            if (!Directory.Exists(latestPluginPath))
            {
                Console.WriteLine("Plugin not found.");
                return null;
            }

            var latestPluginManifestPath = Path.Combine(latestPluginPath, Definitions.ManifestIdentifier);

            var identity = CreatePluginIdentity(latestPluginManifestPath);
            // 顺便检查一下文件夹版本号和manifest版本号是否一致，不一致抛异常。
            //(identity as MaybePackage<RiftPackage>).

            return identity;
        }
        else
        {
            if (!SemVersion.TryParse(descriptor.Version, out var userDefinedVersion))
            {
                Console.WriteLine($"Incorrect version input => `{descriptor.Name}`: `{descriptor.Version}`");
                return null;
            }

            var selectedPluginPath = Path.Combine(pluginPath, userDefinedVersion.ToString());
            if (!Directory.Exists(selectedPluginPath))
            {
                Console.WriteLine("Plugin not found.");
                return null;
            }

            // eg: ~/.rift/plugins/rift.generate/1.0.0/Rift.toml
            var selectedPluginManifestPath = Path.Combine(selectedPluginPath, Definitions.ManifestIdentifier);

            var identity = CreatePluginIdentity(selectedPluginManifestPath);
            return identity;
        }
    }

    private void RetrievePluginDependencies(PluginIdentity identity)
    {
        var scriptPath = identity.Value.Dependencies;
        if (scriptPath is null)
        {
            return;
        }
        ScriptManager.Instance.EvaluateScript(scriptPath);
    }

    private void RetrievePluginMetadata(PluginIdentity identity)
    {
        var scriptPath = identity.Value.Configure;
        if (scriptPath is null)
        {
            return;
        }
        ScriptManager.Instance.EvaluateScript(scriptPath);
    }

    public bool AddDependencyForPlugin(IPackageImportDeclarator declarator)
    {
        if (_currentEvaluatingIdentity is null)
        {
            return false;
        }

        _currentEvaluatingIdentity.Dependencies.Add(declarator.Name, declarator);

        return true;
    }

    public bool AddDependencyForPlugin(IEnumerable<IPackageImportDeclarator> declarators)
    {
        if (_currentEvaluatingIdentity is null)
        {
            return false;
        }

        foreach (var declarator in declarators)
        {
            _currentEvaluatingIdentity.Dependencies.Add(declarator.Name, declarator);
        }

        return true;
    }

    public bool AddMetadataForPlugin(string key, object value)
    {
        if (_currentEvaluatingIdentity is null)
        {
            return false;
        }

        _currentEvaluatingIdentity.Metadata.Add(key, value);
        return true;
    }

}