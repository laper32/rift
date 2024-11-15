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

    [DataMember(Name = "plugin")] 
    public TomlPlugin? Plugin { get; set; }

    [DataMember(Name = "task")]
    public Dictionary<
        string,  // 这个task的名字
        TomlTask // 这个task的实例
    >? Task { get; set; }
}
