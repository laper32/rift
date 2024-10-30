// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Workspace;

internal class Package(IManifest manifest, string manifestPath)
{
    public string Name         => manifest.Name;
    public string ManifestPath => manifestPath;
    public string Root         => Directory.GetParent(ManifestPath)!.FullName;

    public string? Dependencies
    {
        get
        {
            if (manifest.Dependencies is { } dependencies)
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
            if (manifest.Plugins is { } plugins)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, plugins));
            }

            return null;
        }
    }

    public string? Metadata
    {
        get
        {
            if (manifest.Metadata is { } metadata)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, metadata));
            }

            return null;
        }
    }
}
