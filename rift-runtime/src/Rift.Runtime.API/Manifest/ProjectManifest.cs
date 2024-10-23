// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public record ProjectManifest(
    string Name,
    List<string> Authors,
    string Version,
    string Description,
    string? Plugins,
    string? Dependencies,
    string? Metadata,

    List<string> Members,
    List<string> Excludes
    );