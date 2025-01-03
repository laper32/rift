﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Manifest.Virtual;

internal sealed record WorkspaceManifest(
    string                          Name,
    List<string>                    Members,
    List<string>                    Exclude,
    string?                         Plugins,
    string?                         Configure,
    string?                         Dependencies,
    Dictionary<string, JsonElement> Others
);