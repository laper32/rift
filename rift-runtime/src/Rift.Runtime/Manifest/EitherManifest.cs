// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Manifest;

//public class EitherManifest<T>(T manifest) : IEitherManifest<T>
//{
//    public IEitherManifest<T>.EManifestType ManifestType { get; init; } = manifest switch
//    {
//        WorkspaceManifest or FolderManifest => IEitherManifest<T>.EManifestType.Virtual,
//        TargetManifest or ProjectManifest => IEitherManifest<T>.EManifestType.Real,
//        _ => throw new InvalidOperationException("Invalid manifest")
//    };

//    public T Manifest { get; } = manifest;
//}