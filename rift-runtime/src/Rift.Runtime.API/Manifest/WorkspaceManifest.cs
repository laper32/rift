// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public record WorkspaceManifest(
    string Name,
    List<string> Members,
    List<string> Exclude,
    string? Plugins,
    string? Metadata,
    string? Dependencies
    );