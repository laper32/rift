// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

public interface IEitherManifest
{
    public string Name { get; }
}

public enum EManifestType
{
    Virtual,
    Real
}

public record EitherManifest<T> : IEitherManifest
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
        Data = manifest;
    }

    //[JsonIgnore]
    public T Data { get; init; }

    public EManifestType Type { get; init; }

    public string Name => Data switch
    {
        IManifest real => real.Name,
        IVirtualManifest virtualManifest => virtualManifest.Name,
        _ => throw new ArgumentException("Invalid manifest type.")
    };
}