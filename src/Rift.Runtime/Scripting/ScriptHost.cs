using Rift.Runtime.API.Scripting;

namespace Rift.Runtime.Scripting;

public class ScriptHost : IScriptHost
{
    public void Call()
    {
        Console.WriteLine("ScriptHost.Call Initialized...");
    }
}