// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.Manifest.Virtual;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Workspace.Fundamental;

internal class VirtualPackage(IVirtualManifest virtualManifest, string manifestPath)
{
    public string           Name         => virtualManifest.Name;
    public string           Version      => virtualManifest.Version;
    public string           ManifestPath => manifestPath;
    public string           Root         => Directory.GetParent(ManifestPath)!.FullName;
    public IVirtualManifest Value        => virtualManifest;

    public string? Dependencies
    {
        get
        {
            if (virtualManifest.Dependencies is { } dependencies)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies));
            }

            return null;
        }
    }

    public string? Plugins
    {
        get
        {
            if (virtualManifest.Plugins is { } plugins)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, plugins));
            }

            return null;
        }
    }

    public string? Configure
    {
        get
        {
            if (virtualManifest.Configure is { } metadata)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, metadata));
            }

            return null;
        }
    }

    public Dictionary<string, JsonElement> Others => virtualManifest.Others;
}