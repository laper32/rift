// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;
using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

internal class Manifest<T> : IManifest
{
    public Manifest(T manifest)
    {
        if (manifest is not (TargetManifest or ProjectManifest))
        {
            throw new ArgumentException("Manifest must be of type TargetManifest or ProjectManifest");
        }

        Type = manifest switch
        {
            TargetManifest => EManifest.Target,
            ProjectManifest => EManifest.Project,
            _ => throw new ArgumentException("Manifest must be of type TargetManifest or ProjectManifest")
        };

        Value = manifest;
    }

    public EManifest Type { get; init; }

    [JsonIgnore]
    public T Value { get; init; }

    public string Name => Value switch
    {
        ProjectManifest project => project.Name,
        TargetManifest target => target.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Version => Value switch
    {
        ProjectManifest project => project.Version,
        TargetManifest          => null,
        _                       => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Dependencies => Value switch
    {
        ProjectManifest project => project.Dependencies,
        TargetManifest target => target.Dependencies,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Plugins => Value switch
    {
        ProjectManifest project => project.Plugins,
        TargetManifest target => target.Plugins,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Configure => Value switch
    {
        ProjectManifest project => project.Configure,
        TargetManifest target => target.Configure,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public Dictionary<string, JsonElement> Others => Value switch
    {
        ProjectManifest project => project.Others,
        TargetManifest target => target.Others,
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}