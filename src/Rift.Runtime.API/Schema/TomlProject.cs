// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.API.Schema;

// ReSharper disable once IdentifierTypo
public sealed class TomlProject
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }

    [JsonPropertyName("authors")]
    public required List<string> Authors { get; set; }

    [JsonPropertyName("version")]
    public required string Version { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("plugins")]
    public string? Plugins { get; set; }

    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonPropertyName("configure")]
    public string? Configure { get; set; }

    [JsonPropertyName("members")]
    public List<string>? Members { get; set; }

    [JsonPropertyName("exclude")]
    public List<string>? Exclude { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement> Others { get; set; } = [];
}