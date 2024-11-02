using System.Diagnostics.CodeAnalysis;
using System.Text.Json;
using Rift.Runtime.Plugin;

namespace Rift.Runtime.Workspace;

internal class PackageInstance(IMaybePackage package)
{
    public IMaybePackage Value { get; init; } = package;

    public Dictionary<string, object> Metadata { get; init; } = [];
    public Dictionary<string, object> Dependencies { get; init; } = [];
    public Dictionary<string, Scripting.Plugin> Plugins { get; init; } = [];
}

internal class PackageInstances
{
    private readonly Dictionary<
        string,         // InstanceName
        PackageInstance // Instance
    > _value = [];

    public void Add(string packageName, PackageInstance instance)
    {
        _value.Add(packageName, instance);
    }

    public bool TryGetValue(string packageName, [MaybeNullWhen(false)] out PackageInstance ret)
    {
        if (!_value.TryGetValue(packageName, out var instance))
        {
            ret = null;
            return false;
        }

        ret = instance;
        return true;
    }

    public PackageInstance? FindPackageFromManifestPath(string manifestPath)
    {
        var packageInstance = _value.Values.FirstOrDefault(x =>
        {
            var isManifestPathEquals = x.Value.ManifestPath.Equals(manifestPath, StringComparison.Ordinal);
            return isManifestPathEquals;
        });

        return packageInstance;
    }

    public PackageInstance? FindPackageFromScriptPath(string scriptPath)
    {
        var packageInstance = _value.Values.FirstOrDefault(x =>
        {
            var canonicalizedPath = Path.GetFullPath(scriptPath);
            var isPlugin = x.Value.Plugins?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isDependency = x.Value.Dependencies?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isMetadata = x.Value.Metadata?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            return isPlugin || isDependency || isMetadata;
        });
        return packageInstance;
    }

    public void DumpInstancesMetadata()
    {
        Console.WriteLine("DumpInstancesMetadata...");
        var str = JsonSerializer.Serialize(_value, new JsonSerializerOptions
        {
            WriteIndented = true
        });
        Console.WriteLine(str);
        Console.WriteLine("...End");
    }

    public List<PluginDeclarator> CollectPluginsForLoad()
    {
        var result = new List<PluginDeclarator>();

        foreach (var (packageName, instance) in _value)
        {
            if (instance.Plugins.Count <= 0)
            {
                continue;
            }

            foreach (var (pluginName, plugin) in instance.Plugins)
            {
                if (plugin is null)
                {
                    throw new InvalidOperationException($"{pluginName}'s instance is null.");
                }

                var trimmedPluginName = pluginName.Trim();

                if (string.IsNullOrEmpty(trimmedPluginName))
                {
                    // TODO: Warning here, use workspaceManager's logger.
                    Console.WriteLine($"Warning: found a plugin name is empty, the package: `{packageName}`");
                    continue;
                }

                var trimmedPluginVersion = plugin.Version.Trim();
                if (string.IsNullOrEmpty(trimmedPluginVersion))
                {
                    trimmedPluginVersion = "latest";
                }

                result.Add(new PluginDeclarator(trimmedPluginName, trimmedPluginVersion));

            }
        }

        return result;
    }
}