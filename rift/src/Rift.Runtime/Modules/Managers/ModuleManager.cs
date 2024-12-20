using Rift.Runtime.Fundamental;
using Rift.Runtime.Modules.Loader;

namespace Rift.Runtime.Modules.Managers;

public sealed class ModuleManager
{
    private const string ModulePattern = "Rift.Module.*.dll";

    private readonly ModuleAssemblyContext _sharedContext          = new();
    private readonly List<string>          _pendingLoadModulePaths = [];

    private static ModuleManager _instance = null!;
    public ModuleManager()
    {
        _instance = this;
    }

    internal static bool Init()
    {
        _instance.LoadKernelModules();
        return true;
    }

    internal static void Shutdown()
    {

    }
    
    /// <summary>
    /// 加载内核模块 <br />
    /// 内核模块一定和exe放一起的.
    /// </summary>
    private void LoadKernelModules()
    {
        var exePath = ApplicationHost.ExecutablePath;
        var binPath = Directory.GetParent(exePath)!.FullName;

        foreach (var file in Directory.GetFiles(binPath, ModulePattern))
        {
            _pendingLoadModulePaths.Add(file);
        }
    }
}