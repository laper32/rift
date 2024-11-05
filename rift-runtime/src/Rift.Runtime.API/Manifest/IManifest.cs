// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public interface IManifest
{
    public string  Name    { get; }
    /// <summary>
    /// Target不需要版本号，其一定是latest
    /// </summary>
    public string? Version { get; } 
    public string? Dependencies { get; }
    public string? Plugins { get; }
    public string? Metadata { get; }
}