// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Manifest.Real;

/// <summary>
/// Enum representing the type of manifest.
/// </summary>
internal enum EManifest
{
    /// <summary>
    /// Represents a target manifest.
    /// </summary>
    Target,

    /// <summary>
    /// Represents a project manifest.
    /// </summary>
    Project
}

/// <summary>
/// Interface representing a manifest.
/// </summary>
internal interface IManifest
{
    /// <summary>
    /// Gets the type of the manifest.
    /// </summary>
    public EManifest Type { get; }

    /// <summary>
    /// Gets the name of the manifest.
    /// </summary>
    public string Name { get; }

    /// <summary>
    /// Gets the version of the manifest. <br/>
    ///<remarks>
    /// Target manifests do not require a version number and are always considered latest.
    /// </remarks>
    /// </summary>
    public string? Version { get; }

    /// <summary>
    /// Gets the dependencies script path of the manifest.
    /// </summary>
    public string? Dependencies { get; }

    /// <summary>
    /// Gets the plugins script path of the manifest.
    /// </summary>
    public string? Plugins { get; }

    /// <summary>
    /// Gets the configuration script path of the manifest.
    /// </summary>
    public string? Configure { get; }

    /// <summary>
    /// Gets other properties of the manifest.
    /// </summary>
    public Dictionary<string, JsonElement> Others { get; }
}

/// <summary>
/// Class representing a manifest with a generic type.
/// </summary>
/// <typeparam name="T">The type of the manifest, either <see cref="ProjectManifest"/> or <see cref="TargetManifest"/>.</typeparam>
internal class Manifest<T> : IManifest
{
    /// <summary>
    /// Initializes a new instance of the <see cref="Manifest{T}"/> class.
    /// </summary>
    /// <param name="manifest">The manifest object.</param>
    /// <exception cref="ArgumentException">Thrown when the manifest is not of type <see cref="ProjectManifest"/> or <see cref="TargetManifest"/>.</exception>
    public Manifest(T manifest)
    {
        if (manifest is not (TargetManifest or ProjectManifest))
        {
            throw new ArgumentException("Manifest must be of type TargetManifest or ProjectManifest");
        }

        Type = manifest switch
        {
            TargetManifest => EManifest.Target,
            ProjectManifest => EManifest.Project,
            _ => throw new ArgumentException("Manifest must be of type TargetManifest or ProjectManifest")
        };

        Value = manifest;
    }

    /// <summary>
    /// Gets the manifest object.
    /// </summary>
    [JsonIgnore]
    public T Value { get; init; }

    /// <summary>
    /// Gets the type of the manifest.
    /// </summary>
    public EManifest Type { get; init; }

    /// <summary>
    /// Gets the name of the manifest.
    /// </summary>
    public string Name => Value switch
    {
        ProjectManifest project => project.Name,
        TargetManifest target => target.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    /// <summary>
    /// Gets the version of the manifest. <br />
    /// Target's version will always be <b>'latest'</b>
    /// </summary>
    public string Version => Value switch
    {
        ProjectManifest project => project.Version,
        TargetManifest => "latest",
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    /// <summary>
    /// Gets the dependencies script path of the manifest.
    /// </summary>
    public string? Dependencies => Value switch
    {
        ProjectManifest project => project.Dependencies,
        TargetManifest target => target.Dependencies,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    /// <summary>
    /// Gets the plugins script path of the manifest.
    /// </summary>
    public string? Plugins => Value switch
    {
        ProjectManifest project => project.Plugins,
        TargetManifest target => target.Plugins,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    /// <summary>
    /// Gets the configuration script path of the manifest.
    /// </summary>
    public string? Configure => Value switch
    {
        ProjectManifest project => project.Configure,
        TargetManifest target => target.Configure,
        _ => throw new ArgumentException("Invalid manifest type.")
    };

    /// <summary>
    /// Gets other properties of the manifest.
    /// </summary>
    public Dictionary<string, JsonElement> Others => Value switch
    {
        ProjectManifest project => project.Others,
        TargetManifest target => target.Others,
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}
