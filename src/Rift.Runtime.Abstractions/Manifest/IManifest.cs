// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Abstractions.Manifest;

public enum EManifest
{
    Target,
    Project
}

public interface IManifest
{
    public EManifest Type { get; }
    public string    Name { get; }
    /// <summary>
    /// Target不需要版本号，其一定是latest
    /// </summary>
    public string? Version { get; } 
    public string? Dependencies { get; }
    public string? Plugins { get; }
    public string? Configure { get; }

    public Dictionary<string, JsonElement> Others { get; }
}