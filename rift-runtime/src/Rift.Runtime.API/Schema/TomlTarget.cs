// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;

namespace Rift.Runtime.API.Schema;


// ReSharper disable IdentifierTypo
public class TomlTarget
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
}