// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Scripting;

namespace Rift.Runtime.Scripting;

public static class Dependencies
{
    public static void Add<T>(T dependency) where T: class, IPackageDependency
    {

    }
}