// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Scripting;

public interface IScriptManager
{
    public static IScriptManager Instance { get; protected set; } = null!;

    public void EvaluateScript(string scriptPath, int timedOutUnitSec = 15);

    public void AddLibrary(string library);

    public void AddLibrary(IEnumerable<string> libraries);

    public void RemoveLibrary(string library);

    public void RemoveLibrary(IEnumerable<string> libraries);


    public void AddNamespace(string @namespace);

    public void AddNamespace(IEnumerable<string> namespaces);

    public void RemoveNamespace(string @namespace);

    public void RemoveNamespace(IEnumerable<string> namespaces);
}