// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public interface IVirtualManifest
{
    public string Name { get; }
    public List<string> Members { get; }
    public List<string> Excludes { get; }
    public string? Dependencies { get; }
    public string? Plugins { get; }
    public string? Metadata { get; }
}
