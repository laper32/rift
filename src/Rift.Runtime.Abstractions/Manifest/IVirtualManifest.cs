// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Abstractions.Manifest;

public enum EVirtualManifest
{
    Folder,
    Workspace
}

public interface IVirtualManifest
{
    public EVirtualManifest                Type         { get; }
    public string                          Name         { get; }
    public List<string>                    Members      { get; }
    public List<string>                    Exclude      { get; }
    public string?                         Dependencies { get; }
    public string?                         Plugins      { get; }
    public string?                         Configure    { get; }
    public Dictionary<string, JsonElement> Others       { get; }
}
