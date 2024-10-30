// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

internal enum EManifestType
{
    Virtual,
    Real
}

internal interface IEitherManifest
{
    public string Name { get; }
    public EManifestType Type { get; }
}



internal record EitherManifest<T> : IEitherManifest
{
    public EitherManifest(T manifest)
    {
        if (manifest is not (IManifest or IVirtualManifest))
        {
            throw new InvalidOperationException("Only accepts `IManifest` or `IVirtualManifest`");
        }

        Type = manifest switch
        {
            IVirtualManifest => EManifestType.Virtual,
            IManifest => EManifestType.Real,
            _ => throw new ArgumentOutOfRangeException(nameof(manifest), manifest,
                "Only accepts `VirtualManifest` or `Manifest`")
        };
        Value = manifest;
    }

    //[JsonIgnore]
    public T Value { get; init; }

    public EManifestType Type { get; init; }

    public string Name => Value switch
    {
        IManifest real => real.Name,
        IVirtualManifest virtualManifest => virtualManifest.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}