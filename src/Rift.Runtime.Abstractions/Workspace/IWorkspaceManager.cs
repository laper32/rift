// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Abstractions.Workspace;

public interface IWorkspaceManager
{
    string Root { get; }

    IPackageInstance? FindPackage(string name);

    IEnumerable<IPackageInstance> GetAllPackages();
}