// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Scripting;

public interface IScriptSystem
{
    public static IScriptSystem Instance { get; protected set; } = null!;

    public void EvaluateScript(string scriptPath, int timedOutUnitSec = 15);
    
    public void AddLibrary(string library);
    
    public void AddLibrary(IEnumerable<string> libraries);

    public void AddNamespace(string @namespace);

    public void AddNamespace(IEnumerable<string> namespaces);
}