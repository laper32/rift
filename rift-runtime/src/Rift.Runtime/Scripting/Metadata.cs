// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Scripting;

namespace Rift.Runtime.Scripting;

public class Metadata
{
    public static void Add(string key, object value)
    {

    }
    public static void Call()
    {
        var scriptSystem = (IScriptSystemInternal)IScriptSystem.Instance;
        if (scriptSystem.ScriptContext is not { } ctx)
        {
            throw new InvalidOperationException("This function is only allowed in package dependency script.");
        }

        Console.WriteLine($"Metadata.Call invoked, path => {ctx.Path}");
    }
}