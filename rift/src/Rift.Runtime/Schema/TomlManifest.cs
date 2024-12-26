// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;
using Rift.Runtime.Schema.Real;
using Rift.Runtime.Schema.Rift;
using Rift.Runtime.Schema.Virtual;

namespace Rift.Runtime.Schema;

internal sealed class TomlManifest
{
    [JsonPropertyName("target")]
    public TomlTarget? Target { get; set; }

    [JsonPropertyName("project")]
    public TomlProject? Project { get; set; }

    [JsonPropertyName("folder")]
    public TomlFolder? Folder { get; set; }

    [JsonPropertyName("workspace")]
    public TomlWorkspace? Workspace { get; set; }

    [JsonPropertyName("plugin")]
    public TomlPlugin? Plugin { get; set; }
}