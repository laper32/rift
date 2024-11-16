// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Workspace;

public class RiftPackage(IRiftManifest riftManifest, string manifestPath)
{
    public string Name => riftManifest.Name;
    public string ManifestPath => manifestPath;
    public IRiftManifest Value => riftManifest;

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

    public string? Metadata
    {
        get
        {
            if (riftManifest.Metadata is { } metadata)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, metadata));
            }

            return null;
        }
    }
}
