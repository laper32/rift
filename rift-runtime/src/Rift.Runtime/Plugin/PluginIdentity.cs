using System.Text.Json;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.Manifest;
using Rift.Runtime.Workspace;
using Semver;

// 我感觉逃不了课的，不管怎么操作最后都要回到扫一遍你有没有这个插件包

namespace Rift.Runtime.Plugin;

internal class PluginIdentity(IMaybePackage package)
{
    public IMaybePackage              Value        { get; init; } = package;
    public Dictionary<string, object> Dependencies { get; init; } = [];
    public Dictionary<string, object> Metadata     { get; init; } = [];
    public string                     Location => Path.GetFullPath(Directory.GetParent(Value.ManifestPath)!.FullName);
}

internal class PluginIdentities
{
    private const string PluginDirectoryName = "plugins";

    private readonly List<PluginIdentity> _identities = [];

    private PluginIdentity? _currentEvaluatingIdentity;

    private readonly List<string> _pluginSearchPaths =
    [
        Path.Combine(IRuntime.Instance.InstallationPath, PluginDirectoryName), // Rift安装路径
        Path.Combine(IRuntime.Instance.UserPath, PluginDirectoryName), // 用户目录。
    ];

    private PluginIdentity CreatePluginIdentity(string manifestPath)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EManifestType.Rift:
            {
                return manifest switch
                {
                    // 插件是存在一个文件夹里通过版本号作为文件夹区分方式的，所以不能直接打死！
                    EitherManifest<RiftManifest<PluginManifest>> pluginManifest => new PluginIdentity(
                        new MaybePackage<RiftPackage>(new RiftPackage(pluginManifest.Value, manifestPath))),
                    _ => throw new InvalidOperationException("Only supports Rift specific manifests.")
                };
            }
            case EManifestType.Virtual:
            case EManifestType.Real:
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
            if (FindFromSearchPath(x, descriptor) is { } identity)
            {
                possiblePlugins.Add(identity);
            }
        });
        if (possiblePlugins.Count == 0)
        {
            return;
        }
        var possiblePlugin = possiblePlugins.First();
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
            WriteIndented = true, PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
        }));
    }

    private void AnalyzeDependencies(PluginIdentity identity)
    {

        identity.Dependencies.ForEach((key, value) =>
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
        IScriptManager.Instance.EvaluateScript(scriptPath);
    }

    private void RetrievePluginMetadata(PluginIdentity identity)
    {
        var scriptPath = identity.Value.Metadata;
        if (scriptPath is null)
        {
            return;
        }
        IScriptManager.Instance.EvaluateScript(scriptPath);
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