// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Manifest;

public sealed record PluginManifest(
    string                          Name,
    List<string>                    Authors,
    string                          Version,
    string?                         Description,
    string?                         Configure,
    string?                         Dependency,
    Dictionary<string, JsonElement> Others
);