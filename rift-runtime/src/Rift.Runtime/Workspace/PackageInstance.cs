// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics.CodeAnalysis;
using System.Text.Json;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Plugin;

namespace Rift.Runtime.Workspace;

internal class PackageInstance(IMaybePackage package) : IPackageInstance
{
    public IMaybePackage Value { get; init; } = package;

    public Dictionary<string, object>           Metadata     { get; init; } = [];
    public Dictionary<string, object>           Dependencies { get; init; } = [];
    public Dictionary<string, Scripting.Plugin> Plugins      { get; init; } = [];

    public string        Name         => Value.Name;
    public string        ManifestPath => Value.ManifestPath;

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
            var isPlugin = x.Value.Plugins?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isDependency = x.Value.Dependencies?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            var isConfigure = x.Value.Configure?.Equals(canonicalizedPath, StringComparison.Ordinal) ?? false;
            return isPlugin || isDependency || isConfigure;
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
                    // TODO: Warning here, use workspaceManager's logger.
                    Console.WriteLine($"Warning: found a plugin name is empty, the package: `{packageName}`");
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