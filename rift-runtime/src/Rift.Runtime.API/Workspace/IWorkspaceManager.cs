// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Workspace;

public interface IWorkspaceManager
{
    public static IWorkspaceManager Instance { get; protected set; } = null!;

    /// <summary>
    /// 项目根目录
    /// </summary>
    public string Root { get; }

    void PrintMessage();


}