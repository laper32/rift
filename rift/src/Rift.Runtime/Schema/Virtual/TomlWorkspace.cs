﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Schema.Virtual;

internal sealed class TomlWorkspace
{
    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("members")]
    public List<string>? Members { get; set; }

    [JsonPropertyName("exclude")]
    public List<string>? Exclude { get; set; }

    [JsonPropertyName("plugins")]
    public string? Plugins { get; set; }

    [JsonPropertyName("configure")]
    public string? Configure { get; set; }

    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement> Others { get; set; } = [];
}