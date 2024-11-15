// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;


// ReSharper disable once IdentifierTypo
public class TomlWorkspace
{
    [DataMember(Name = "name")]
    public string? Name { get; set; }

    [DataMember(Name = "members")]
    public List<string>? Members { get; set; }

    [DataMember(Name = "exclude")]
    public List<string>? Exclude { get; set; }

    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }

    [DataMember(Name = "configure")]
    public string? Configure { get; set; }

    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
}
