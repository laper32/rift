﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json.Serialization;

namespace Rift.Runtime.Schema.Virtual;

internal class TomlFolder
{
    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("members")]
    public List<string>? Members { get; set; }

    [JsonPropertyName("exclude")]
    public List<string>? Exclude { get; set; }
}