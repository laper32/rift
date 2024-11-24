// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Workspace;

public interface IWorkspaceManager
{
    string Root { get; }

    IPackageInstance? FindPackage(string name);

    IEnumerable<IPackageInstance> GetAllPackages();
}

public abstract class WorkspaceManager
{
    protected WorkspaceManager()
    {
        Instance = this;
    }

    public static WorkspaceManager Instance { get; protected set; } = null!;

    /// <summary>
    /// 项目根目录
    /// </summary>
    public abstract string Root { get; protected set; }

    public abstract IPackageInstance? FindPackage(string name);
    public abstract IEnumerable<IPackageInstance> GetAllPackages();
}