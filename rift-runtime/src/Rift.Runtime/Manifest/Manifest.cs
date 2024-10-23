// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

public class Manifest : IManifest
{
    public string Name { get; internal set; } = null!;
    public string? Dependencies { get; internal set; }
    public string? Plugins { get; internal set; }
    public string? Metadata { get; internal set; }

    
}