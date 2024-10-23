// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

// ReSharper disable IdentifierTypo
// ReSharper disable StringLiteralTypo
public class TomlManifest
{
    [DataMember(Name = "target")] 
    public TomlTarget? Target { get; set; }
    
    [DataMember(Name = "project")]
    public TomlProject? Project { get; set; }
    
    [DataMember(Name = "folder")]
    public TomlFolder? Folder { get; set; }

    [DataMember(Name = "workspace")]
    public TomlWorkspace? Workspace { get; set; }
}
