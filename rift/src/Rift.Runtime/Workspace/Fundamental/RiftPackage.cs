﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.Manifest.Rift;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Workspace.Fundamental;

internal class RiftPackage(IRiftManifest riftManifest, string manifestPath)
{
    public string        Name         => riftManifest.Name;
    public string        ManifestPath => manifestPath;
    public string        Root         => Directory.GetParent(ManifestPath)!.FullName;
    public IRiftManifest Value        => riftManifest;

    public string? Dependencies
    {
        get
        {
            if (riftManifest.Dependencies is { } dependencies)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies));
            }

            return null;
        }
    }

    public string? Configure
    {
        get
        {
            if (riftManifest.Configure is { } metadata)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, metadata));
            }

            return null;
        }
    }

    public Dictionary<string, JsonElement> Others => riftManifest.Others;
}