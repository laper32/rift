// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.API.Schema;

public class TomlPlugin
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }

    [JsonPropertyName("version")]
    public required string Version { get; set; }
    
    [JsonPropertyName("authors")]
    public required List<string> Authors { get; set; }
    
    [JsonPropertyName("description")]
    public string? Description { get; set; }
    
    [JsonPropertyName("configure")]
    public string? Configure { get; set; }
    
    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonExtensionData] public Dictionary<string, JsonElement> Others { get; set; } = [];
}
