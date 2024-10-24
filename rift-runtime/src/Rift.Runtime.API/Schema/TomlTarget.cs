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
    public string Name { get; set; } = null!;
    [DataMember(Name = "type")]
    public string Type { get; set; } = null!;
    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }
    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
    [DataMember(Name = "metadata")]
    public string? Metadata { get; set; }
}