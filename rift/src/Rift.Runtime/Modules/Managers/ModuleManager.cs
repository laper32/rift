using Rift.Runtime.Modules.Loader;

namespace Rift.Runtime.Modules.Managers;

public sealed class ModuleManager
{
    private readonly ModuleAssemblyContext _sharedContext = new();

    internal static bool Init()
    {
        return true;
    }

    internal static void Shutdown()
    {

    }

    private void LoadKernelModules()
    {

    }
}