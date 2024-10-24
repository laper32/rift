// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;
using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

public record Manifest<T> : IManifest
{
    public Manifest(T manifest)
    {
        if (manifest is not (TargetManifest or ProjectManifest))
        {
            throw new ArgumentException("Manifest must be of type TargetManifest or ProjectManifest");
        }

        Data = manifest;
    }

    [JsonIgnore]
    public T Data { get; init; }

    public string Name => Data switch
    {
        ProjectManifest project => project.Name,
        TargetManifest target => target.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Dependencies => Data switch
    {
        ProjectManifest project => project.Dependencies,
        TargetManifest target => target.Dependencies,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Plugins => Data switch
    {
        ProjectManifest project => project.Plugins,
        TargetManifest target => target.Plugins,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Metadata => Data switch
    {
        ProjectManifest project => project.Metadata,
        TargetManifest target => target.Metadata,
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}