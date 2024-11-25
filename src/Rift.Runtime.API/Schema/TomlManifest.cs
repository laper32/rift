// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;

namespace Rift.Runtime.API.Schema;

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

    [JsonPropertyName("task")]
    public Dictionary<
        string,  // 这个task的名字
        TomlTask // 这个task的实例
    >? Task { get; set; }
}
