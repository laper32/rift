// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;


// ReSharper disable IdentifierTypo
public class TomlTarget
{
    [DataMember(Name = "name")]
    public required string Name { get; set; }
    [DataMember(Name = "type")]
    public required string Type { get; set; }
    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }
    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
    [DataMember(Name = "metadata")]
    public string? Metadata { get; set; }
}