// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Workspace;

internal interface IMaybePackage
{
    public enum EMaybePackage
    {
        Package,
        Virtual
    }

    public string  Name         { get; }
    public string? Dependencies { get; }
    public string? Plugins      { get; }
    public string? Metadata     { get; }
    public string  ManifestPath { get; }
}

internal class MaybePackage<T>(T value) : IMaybePackage
{
    public IMaybePackage.EMaybePackage Type { get; init; } = value switch
    {
        Package        => IMaybePackage.EMaybePackage.Package,
        VirtualPackage => IMaybePackage.EMaybePackage.Virtual,
        _              => throw new InvalidOperationException("Only accepts `Package` or `VirtualPackage`.")
    };

    public T Value { get; init; } = value;

    public string ManifestPath => Value switch
    {
        Package package        => package.ManifestPath,
        VirtualPackage package => package.ManifestPath,
        _                      => throw new InvalidOperationException("Why you at here?")
    };

    public string Name => Value switch
    {
        Package package        => package.Name,
        VirtualPackage package => package.Name,
        _                      => string.Empty
    };

    public string? Dependencies => Value switch
    {
        Package package        => package.Dependencies,
        VirtualPackage package => package.Dependencies,
        _                      => null
    };

    public string? Plugins => Value switch
    {
        Package package        => package.Plugins,
        VirtualPackage package => package.Plugins,
        _                      => null
    };

    public string? Metadata => Value switch
    {
        Package package        => package.Metadata,
        VirtualPackage package => package.Metadata,
        _                      => null
    };
}
