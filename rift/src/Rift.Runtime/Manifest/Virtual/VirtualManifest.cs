﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Manifest.Virtual;

internal enum EVirtualManifest
{
    Folder,
    Workspace
}

internal interface IVirtualManifest
{
    /// <summary>
    ///     Manifest type.
    /// </summary>
    public EVirtualManifest Type { get; }

    /// <summary>
    ///     Manifest name.
    /// </summary>
    public string Name { get; }

    /// <summary>
    ///     Virtual manifest's version will always be latest.
    /// </summary>
    public string Version { get; }

    public List<string> Members { get; }

    public List<string> Exclude { get; }

    public string? Dependencies { get; }

    public string? Plugins { get; }

    public string? Configure { get; }

    public Dictionary<string, JsonElement> Others { get; }
}

internal class VirtualManifest<T> : IVirtualManifest
{
    public VirtualManifest(T manifest)
    {
        if (manifest is not (WorkspaceManifest or FolderManifest))
        {
            throw new ArgumentException("Manifest must be of type WorkspaceManifest or FolderManifest");
        }

        Type = manifest switch
        {
            FolderManifest => EVirtualManifest.Folder,
            WorkspaceManifest => EVirtualManifest.Workspace,
            _ => throw new UnreachableException("Manifest must be of type WorkspaceManifest or FolderManifest")
        };

        Value = manifest;
    }

    [JsonIgnore]
    public T Value { get; init; }

    public EVirtualManifest Type { get; init; }

    public string Name => Value switch
    {
        WorkspaceManifest workspace => workspace.Name,
        FolderManifest folder       => folder.Name,
        _                           => throw new UnreachableException()
    };

    public string Version => "latest";

    public List<string> Members => Value switch
    {
        WorkspaceManifest workspace => workspace.Members,
        FolderManifest folder       => folder.Members,
        _                           => throw new UnreachableException()
    };

    public List<string> Exclude => Value switch
    {
        WorkspaceManifest workspace => workspace.Exclude,
        FolderManifest folder => folder.Exclude,
        _ => throw new UnreachableException()
    };

    public string? Dependencies => Value switch
    {
        WorkspaceManifest workspace => workspace.Dependencies,
        FolderManifest              => throw new ArgumentException("[folder] does not have `dependencies` field."),
        _                           => throw new UnreachableException()
    };

    public string? Plugins => Value switch
    {
        WorkspaceManifest workspace => workspace.Plugins,
        FolderManifest              => throw new ArgumentException("[folder] does not have `plugins` field."),
        _                           => throw new UnreachableException()
    };

    public string? Configure => Value switch
    {
        WorkspaceManifest workspace => workspace.Configure,
        FolderManifest              => throw new ArgumentException("[folder] does not have `metadata` field."),
        _                           => throw new UnreachableException()
    };

    public Dictionary<string, JsonElement> Others => Value switch
    {
        WorkspaceManifest workspace => workspace.Others,
        FolderManifest => throw new ArgumentException("`[folder]` does not support extension field currently."),
        _ => throw new UnreachableException()
    };
}