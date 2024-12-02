// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Manifest;

public sealed record TargetManifest(
    string Name,
    string Type,
    string? Plugins,
    string? Dependencies,
    string? Configure,
    Dictionary<string, JsonElement> Others);