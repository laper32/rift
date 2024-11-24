// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Scripting;

public interface IScriptManager
{
    void EvaluateScript(string scriptPath, int timedOutUnitSec = 15);

    void AddLibrary(string library);

    void AddLibrary(IEnumerable<string> libraries);

    void RemoveLibrary(string library);

    void RemoveLibrary(IEnumerable<string> libraries);

    void AddNamespace(string @namespace);

    void AddNamespace(IEnumerable<string> namespaces);

    void RemoveNamespace(string @namespace);

    void RemoveNamespace(IEnumerable<string> namespaces);
}