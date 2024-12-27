// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Rift.Runtime.Manifest.Real;
using Rift.Runtime.Manifest.Rift;
using Rift.Runtime.Manifest.Virtual;

namespace Rift.Runtime.Manifest;

/// <summary>
///     Enum representing all possible Manifest types. <br />
///     Simulates the absence of an Enum class.
/// </summary>
internal enum EEitherManifest
{
    Virtual,
    Real,
    Rift
}

/// <summary>
///     Interface for EitherManifest, providing Name and Type properties.
/// </summary>
internal interface IEitherManifest
{
    /// <summary>
    ///     Gets the name of the manifest.
    /// </summary>
    public string Name { get; }

    /// <summary>
    /// Manifest version.
    /// </summary>
    public string Version { get; }

    /// <summary>
    ///     Gets the type of the manifest.
    /// </summary>
    public EEitherManifest Type { get; }
}

/// <summary>
///     A record representing a manifest that can be of type IManifest, IVirtualManifest, or IRiftManifest.
/// </summary>
/// <typeparam name="T">The type of the manifest.</typeparam>
internal record EitherManifest<T> : IEitherManifest
{
    /// <summary>
    ///     Initializes a new instance of the <see cref="EitherManifest{T}"/> class.
    /// </summary>
    /// <param name="manifest">The manifest instance.</param>
    /// <exception cref="InvalidOperationException">Thrown when the manifest is not of a valid type.</exception>
    public EitherManifest(T manifest)
    {
        if (manifest is not (IManifest or IVirtualManifest or IRiftManifest))
        {
            throw new InvalidOperationException("Only accepts `IManifest`, `IVirtualManifest`, or `IRiftManifest`");
        }

        Type = manifest switch
        {
            IVirtualManifest => EEitherManifest.Virtual,
            IManifest => EEitherManifest.Real,
            IRiftManifest => EEitherManifest.Rift,
            _ => throw new ArgumentOutOfRangeException(nameof(manifest), manifest,
                "Only accepts `VirtualManifest`, `Manifest`, or `RiftManifest`")
        };

        Value = manifest;
    }

    /// <summary>
    ///     Gets the manifest value.
    /// </summary>
    //[JsonIgnore]
    public T Value { get; init; }

    /// <summary>
    ///     Gets the type of the manifest.
    /// </summary>
    public EEitherManifest Type { get; init; }

    /// <summary>
    ///     Gets the name of the manifest.
    /// </summary>
    /// <exception cref="UnreachableException">Thrown when the manifest type is invalid.</exception>
    public string Name => Value switch
    {
        IManifest real => real.Name,
        IVirtualManifest virtualManifest => virtualManifest.Name,
        IRiftManifest riftManifest => riftManifest.Name,
        _ => throw new UnreachableException()
    };

    public string Version => Value switch
    {
        IManifest real      => real.Version,
        IVirtualManifest vm => vm.Version,
        IRiftManifest rm    => rm.Version,
        _                   => throw new UnreachableException()
    };
}
