// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Schema;


// ReSharper disable IdentifierTypo
public sealed class TomlTarget
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }
    
    [JsonPropertyName("type")]
    public required string Type { get; set; }
    
    [JsonPropertyName("plugins")]
    public string? Plugins { get; set; }
    
    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonPropertyName("configure")]
    public string? Configure { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement> Others { get; set; } = [];
}