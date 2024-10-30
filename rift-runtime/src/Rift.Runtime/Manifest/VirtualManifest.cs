// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;
using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

public record VirtualManifest<T> : IVirtualManifest
{
    public VirtualManifest(T manifest)
    {
        if (manifest is not (WorkspaceManifest or FolderManifest))
        {
            throw new ArgumentException("Manifest must be of type WorkspaceManifest or FolderManifest");
        }

        Value = manifest;
    }
    [JsonIgnore]
    public T Value { get; init; }

    public string Name => Value switch
    {
        WorkspaceManifest workspace => workspace.Name,
        FolderManifest folder => folder.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public List<string> Members => Value switch
    {
        WorkspaceManifest workspace => workspace.Members,
        FolderManifest folder => folder.Members,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public List<string> Exclude => Value switch
    {
        WorkspaceManifest workspace => workspace.Exclude,
        FolderManifest folder => folder.Exclude,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Dependencies => Value switch
    {
        WorkspaceManifest workspace => workspace.Dependencies,
        FolderManifest              => throw new ArgumentException("[folder] does not have `dependencies` field."),
        _                           => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Plugins => Value switch
    {
        WorkspaceManifest workspace => workspace.Plugins,
        FolderManifest => throw new ArgumentException("[folder] does not have `plugins` field."),
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    public string? Metadata => Value switch
    {
        WorkspaceManifest workspace => workspace.Metadata,
        FolderManifest => throw new ArgumentException("[folder] does not have `metadata` field."),
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}
