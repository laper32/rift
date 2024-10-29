// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.System;

public interface IScriptSystem
{
    public static IScriptSystem Instance { get; protected set; } = null!;

    public void EvaluateScript(string scriptPath, int timedOutUnitSec = 15);

    public void AddLibraries(IEnumerable<string>  libraries);
    public void AddNamespaces(IEnumerable<string> namespaces);
}