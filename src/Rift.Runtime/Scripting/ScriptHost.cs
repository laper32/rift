namespace Rift.Runtime.Scripting;

public interface IScriptHost
{
    void Call();
}

public class ScriptHost : IScriptHost
{
    public void Call()
    {
        Console.WriteLine("ScriptHost.Call Initialized...");
    }
}