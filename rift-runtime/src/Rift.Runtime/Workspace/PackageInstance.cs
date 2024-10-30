using System.Diagnostics.CodeAnalysis;
using System.Text.Json;

namespace Rift.Runtime.Workspace;

internal class PackageInstance(IMaybePackage package)
{
    public IMaybePackage Value { get; init; } = package;

    public Dictionary<string, object> Metadata { get; init; } = [];
    public Dictionary<string, object> Dependencies { get; init; } = [];
    public Dictionary<string, object> Plugins { get; init; } = [];
}

internal class PackageInstances
{

    private readonly Dictionary<string, PackageInstance> _value = [];

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
            var isPlugin          = x.Value.Plugins?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isDependency      = x.Value.Dependencies?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isMetadata        = x.Value.Metadata?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            return isPlugin || isDependency || isMetadata;
        });
        return packageInstance;
    }

    public void DumpPackagesMetadata()
    {
        foreach (var instance in _value)
        {
            var str = JsonSerializer.Serialize(instance, new JsonSerializerOptions
            {
                WriteIndented = true
            });
            Console.WriteLine(str);
        }
    }
}