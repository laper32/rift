namespace Rift.Runtime.Tests;

public class RuntimeSetup : IDisposable
{
    public RuntimeSetup()
    {
        Bootstrap.Init();
    }
    public void Dispose()
    {
        Bootstrap.Shutdown();
    }
}