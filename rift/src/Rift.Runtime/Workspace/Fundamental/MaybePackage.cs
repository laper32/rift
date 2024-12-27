// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Text.Json;

namespace Rift.Runtime.Workspace.Fundamental;

internal enum EMaybePackage
{
    Package,
    Virtual,
    Rift
}

internal interface IMaybePackage
{
    public EMaybePackage                   Type         { get; }
    public string                          Name         { get; }
    public string                          Version      { get; }
    public string?                         Dependencies { get; }
    public string?                         Plugins      { get; }
    public string?                         Configure    { get; }
    public string                          ManifestPath { get; }
    public string                          Root         { get; }
    public Dictionary<string, JsonElement> Others       { get; }
}

internal class MaybePackage<T>(T value) : IMaybePackage
{
    public T Value { get; init; } = value;

    public EMaybePackage Type { get; init; } = value switch
    {
        Package        => EMaybePackage.Package,
        VirtualPackage => EMaybePackage.Virtual,
        RiftPackage    => EMaybePackage.Rift,
        _              => throw new InvalidOperationException("Only accepts `Package` or `VirtualPackage`.")
    };

    public string ManifestPath => Value switch
    {
        Package package        => package.ManifestPath,
        VirtualPackage package => package.ManifestPath,
        RiftPackage package    => package.ManifestPath,
        _                      => throw new UnreachableException()
    };

    public string Root => Value switch
    {
        Package package        => package.Root,
        VirtualPackage package => package.Root,
        RiftPackage package    => package.Root,
        _                      => throw new UnreachableException()
    };

    public string Name => Value switch
    {
        Package package        => package.Name,
        VirtualPackage package => package.Name,
        RiftPackage package    => package.Name,
        _                      => throw new UnreachableException()
    };

    public string Version => Value switch
    {
        Package package        => package.Version,
        VirtualPackage package => package.Version,
        RiftPackage package    => package.Version,
        _                      => throw new UnreachableException()
    };


    public string? Dependencies => Value switch
    {
        Package package        => package.Dependencies,
        VirtualPackage package => package.Dependencies,
        RiftPackage package    => package.Dependencies,
        _                      => null
    };

    public string? Plugins => Value switch
    {
        Package package        => package.Plugins,
        VirtualPackage package => package.Plugins,
        _                      => null
    };

    public string? Configure => Value switch
    {
        Package package        => package.Configure,
        VirtualPackage package => package.Configure,
        RiftPackage package    => package.Configure,
        _                      => null
    };

    public Dictionary<string, JsonElement> Others => Value switch
    {
        Package package        => package.Others,
        VirtualPackage package => package.Others,
        RiftPackage package    => package.Others,
        _                      => []
    };
}