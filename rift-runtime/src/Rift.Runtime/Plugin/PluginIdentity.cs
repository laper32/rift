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

}

internal class PluginIdentities
{
    private const string PluginDirectoryName = "plugins";
    private const string PluginLibraryName = "lib";       // eg. ~/.rift/plugins/Example/lib/Example.dll 

    private readonly List<PluginIdentity> _identities = [];

    private PluginIdentity? _currentEvaluatingIdentity = null;

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
                switch (manifest)
                {
                    // 插件是存在一个文件夹里通过版本号作为文件夹区分方式的，所以不能直接打死！
                    case EitherManifest<RiftManifest<PluginManifest>> pluginManifest:
                    {
                        var riftPackage = new RiftPackage(pluginManifest.Value, manifestPath);
                        return new PluginIdentity(new MaybePackage<RiftPackage>(new RiftPackage(pluginManifest.Value, manifestPath)));
                    }
                    default:
                    {
                        throw new InvalidOperationException("Only supports Rift specific manifests.");
                    }
                }
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
    /// 根据Descriptor找插件 <br/>
    /// 注：<br/>
    /// <remarks>
    ///     1. 插件的名字一定是文件夹的名字，且版本是根据文件夹名来做分类。 <br/>
    ///     2. 通过Rift.toml解析插件包。
    /// </remarks>
    /// </summary>
    /// <param name="descriptor"></param>
    public void FindPlugin(PluginDescriptor descriptor)
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

        uniqueSearchPaths.ForEach(x =>
        {
            FindPluginFromSearchPath(x, descriptor);
        });

    }

    private void FindPluginFromSearchPath(string path, PluginDescriptor descriptor)
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
            Console.WriteLine("No version found, why?");
            return;
        }

        var latestVersion = pluginVersions.Max(SemVersion.SortOrderComparer)!;

        if (descriptor.Version.Equals("latest", StringComparison.OrdinalIgnoreCase))
        {
            var latestPluginPath = Path.Combine(pluginPath, latestVersion.ToString());

            if (!Directory.Exists(latestPluginPath))
            {
                Console.WriteLine("Plugin not found.");
                return;
            }

            var latestPluginManifestPath = Path.Combine(latestPluginPath, Definitions.ManifestIdentifier);

            var identity = CreatePluginIdentity(latestPluginManifestPath);
            _currentEvaluatingIdentity = identity;
            RetrievePluginDependencies(identity);
            RetrievePluginMetadata(identity);
            Console.WriteLine("FindPluginFromSearchPath...");
            Console.WriteLine(JsonSerializer.Serialize(identity, new JsonSerializerOptions
            {
                WriteIndented = true
            }));
            Console.WriteLine("...End");
        }
        else
        {
            if (!SemVersion.TryParse(descriptor.Version, out var userDefinedVersion))
            {
                Console.WriteLine($"Incorrect version input => `{descriptor.Name}`: `{descriptor.Version}`");
                return;
            }

            var selectedPluginPath = Path.Combine(pluginPath, userDefinedVersion.ToString());
            if (!Directory.Exists(selectedPluginPath))
            {
                Console.WriteLine("Plugin not found.");
                return;
            }

            // eg: ~/.rift/plugins/rift.generate/1.0.0/Rift.toml
            var selectedPluginManifestPath = Path.Combine(selectedPluginPath, Definitions.ManifestIdentifier);
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