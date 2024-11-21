// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Scripting;

public abstract class ScriptManager
{
    protected ScriptManager()
    {
        Instance = this;
    }

    public static ScriptManager Instance { get; protected set; } = null!;

    public abstract void EvaluateScript(string scriptPath, int timedOutUnitSec = 15);

    public abstract void AddLibrary(string library);

    public abstract void AddLibrary(IEnumerable<string> libraries);

    public abstract void RemoveLibrary(string library);

    public abstract void RemoveLibrary(IEnumerable<string> libraries);

    public abstract void AddNamespace(string @namespace);

    public abstract void AddNamespace(IEnumerable<string> namespaces);

    public abstract void RemoveNamespace(string @namespace);

    public abstract void RemoveNamespace(IEnumerable<string> namespaces);
}