// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;

namespace Rift.Runtime.Workspace;

public interface IPackageInstance
{
    public string                               Name         { get; }
    public string                               ManifestPath { get; }
    public Dictionary<string, PackageReference> Dependencies { get; }

    JsonElement? GetExtensionField(string name);

    bool HasPlugin(string name);

    bool HasDependency(string name);
}

internal class PackageInstance(IMaybePackage package) : IPackageInstance
{
    public IMaybePackage Value { get; init; } = package;

    public Dictionary<string, object>           Metadata     { get; init; } = [];
    public Dictionary<string, PackageReference> Plugins      { get; init; } = [];
    public Dictionary<string, PackageReference> Dependencies { get; init; } = [];
    public string                               Name         => Value.Name;
    public string                               ManifestPath => Value.ManifestPath;

    public bool HasPlugin(string name)
    {
        return Plugins.Any(x => x.Key.Equals(name, StringComparison.OrdinalIgnoreCase));
    }

    public bool HasDependency(string name)
    {
        return Dependencies.Any(x => x.Key.Equals(name, StringComparison.OrdinalIgnoreCase));
    }

    public JsonElement? GetExtensionField(string name)
    {
        if (Value.Others.TryGetValue(name, out var value))
        {
            return value;
        }

        return null;
    }
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

    public IEnumerable<PackageInstance> GetAllInstances()
    {
        return _value.Values;
    }

    public PackageInstance? FindInstance(string packageName)
    {
        return _value.GetValueOrDefault(packageName);
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
            var isConfigure       = x.Value.Configure?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            return isPlugin || isDependency || isConfigure;
        });
        return packageInstance;
    }

    public void DumpInstancesMetadata()
    {
        Tty.WriteLine("DumpInstancesMetadata...");
        var str = JsonSerializer.Serialize(_value, new JsonSerializerOptions
        {
            WriteIndented = true
        });
        Tty.WriteLine(str);
        Tty.WriteLine("...End");
    }

    public IEnumerable<PluginDescriptor> CollectPluginsForLoad()
    {
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
                    Tty.Warning($"Found a plugin name is empty, package: `{packageName}`");
                    continue;
                }

                var trimmedPluginVersion = plugin.Version.Trim();
                if (string.IsNullOrEmpty(trimmedPluginVersion))
                {
                    trimmedPluginVersion = "latest";
                }

                yield return new PluginDescriptor(trimmedPluginName, trimmedPluginVersion);
            }
        }
    }
}