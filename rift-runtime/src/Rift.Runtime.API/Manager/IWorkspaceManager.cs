// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.API.Manager;

public interface IWorkspaceManager
{
    public static IWorkspaceManager Instance { get; protected set; } = null!;

    /// <summary>
    /// 项目根目录
    /// </summary>
    public string Root { get; }

    void LoadPackage(string path);
    //public IEitherManifest<T>? ReadManifest<T>(string path) where T : class;
}