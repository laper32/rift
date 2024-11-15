// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public record TargetManifest(
    string Name,
    string Type,
    string? Plugins,
    string? Dependencies,
    string? Configure
    );