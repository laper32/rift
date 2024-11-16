// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public record PluginManifest(
    string       Name,
    List<string> Authors,
    string       Version,
    string?      Description,
    string?      Configure,
    string?      Dependency
);
