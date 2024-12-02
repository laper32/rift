// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Manifest;

public enum ERiftManifest
{
    Plugin
}


public interface IRiftManifest
{
    public ERiftManifest            Type         { get; }
    string                          Name         { get; }
    List<string>                    Authors      { get; }
    string                          Version      { get; }
    string?                         Description  { get; }
    string?                         Configure    { get; }
    string?                         Dependencies { get; }
    Dictionary<string, JsonElement> Others       { get; }
}