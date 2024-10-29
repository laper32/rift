// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.API.Manager;
/*
pub enum WorkspaceStatus {
       Unknown,
       Init,
       PackageLoaded,
       Failed,
   }
 */

public enum EWorkspaceStatus
{
    Unknown,
    Init,
    Ready,
    Failed
}

public interface IWorkspaceManager
{
    public static IWorkspaceManager Instance { get; protected set; } = null!;

    public EWorkspaceStatus Status { get; }

    /// <summary>
    /// 项目根目录
    /// </summary>
    public string Root { get; }

    void SetRootPath(string path);
    void LoadWorkspace();
    void PrintMessage();
}