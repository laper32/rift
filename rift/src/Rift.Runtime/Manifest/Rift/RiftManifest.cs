// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Manifest.Rift;

internal enum ERiftManifest
{
    Plugin
}

internal interface IRiftManifest
{
    public ERiftManifest            Type         { get; }
    string                          Name         { get; }
    List<string>                    Authors      { get; }
    string                          Version      { get; }
    string?                         Description  { get; }
    string?                         Configure    { get; }
    string?                         Dependencies { get; }
    Dictionary<string, JsonElement> Others       { get; }
}

internal class RiftManifest<T> : IRiftManifest
{
    public RiftManifest(T manifest)
    {
        if (manifest is not PluginManifest)
        {
            throw new ArgumentException("Manifest must be of type RiftManifest");
        }

        Type = manifest switch
        {
            PluginManifest => ERiftManifest.Plugin,
            _              => throw new UnreachableException("Manifest must be of type RiftManifest")
        };

        Value = manifest;
    }

    [JsonIgnore]
    public T Value { get; init; }

    public ERiftManifest Type { get; init; }

    public string Name => Value switch
    {
        PluginManifest plugin => plugin.Name,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public List<string> Authors => Value switch
    {
        PluginManifest plugin => plugin.Authors,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public string Version => Value switch
    {
        PluginManifest plugin => plugin.Version,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public string? Description => Value switch
    {
        PluginManifest plugin => plugin.Description,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public string? Configure => Value switch
    {
        PluginManifest plugin => plugin.Configure,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public string? Dependencies => Value switch
    {
        PluginManifest plugin => plugin.Dependency,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };

    public Dictionary<string, JsonElement> Others => Value switch
    {
        PluginManifest plugin => plugin.Others,
        _                     => throw new UnreachableException("Invalid manifest type.")
    };
}