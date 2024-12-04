// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;

namespace Rift.Runtime.Schema;

// ReSharper disable IdentifierTypo
// ReSharper disable StringLiteralTypo
public sealed class TomlManifest
{
    //[DataMember(Name = "target")]
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