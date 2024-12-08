// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Manifest;

/// <summary>
///     所有可能的Manifest类型。 <br />
///     没有Enum class的情况下所做的模拟
/// </summary>
internal enum EEitherManifest
{
    Virtual,
    Real,
    Rift
}

internal interface IEitherManifest
{
    public string          Name { get; }
    public EEitherManifest Type { get; }
}

internal record EitherManifest<T> : IEitherManifest
{
    public EitherManifest(T manifest)
    {
        if (manifest is not (IManifest or IVirtualManifest or IRiftManifest))
        {
            throw new InvalidOperationException("Only accepts `IManifest`, `IVirtualManifest`, or `IRiftManifest`");
        }

        Type = manifest switch
        {
            IVirtualManifest => EEitherManifest.Virtual,
            IManifest        => EEitherManifest.Real,
            IRiftManifest    => EEitherManifest.Rift,
            _ => throw new ArgumentOutOfRangeException(nameof(manifest), manifest,
                "Only accepts `VirtualManifest`, `Manifest`, or `RiftManifest`")
        };

        Value = manifest;
    }

    //[JsonIgnore]
    public T Value { get; init; }

    public EEitherManifest Type { get; init; }

    public string Name => Value switch
    {
        IManifest real                   => real.Name,
        IVirtualManifest virtualManifest => virtualManifest.Name,
        IRiftManifest riftManifest       => riftManifest.Name,
        _                                => throw new ArgumentException("Invalid manifest type.")
    };
}