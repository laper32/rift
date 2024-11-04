// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

// ReSharper disable once IdentifierTypo
public class TomlProject
{
    [DataMember(Name = "name")] public required string Name { get; set; }


    [DataMember(Name = "authors")]
    public required List<string> Authors { get; set; }

    [DataMember(Name = "version")]
    public required string Version { get; set; }

    [DataMember(Name = "description")]
    public string? Description { get; set; }

    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }

    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }

    [DataMember(Name = "metadata")]
    public string? Metadata { get; set; }

    [DataMember(Name = "members")]
    public List<string>? Members { get; set; }

    [DataMember(Name = "exclude")]
    public List<string>? Exclude { get; set; }
}