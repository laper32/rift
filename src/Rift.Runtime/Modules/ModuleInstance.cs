using McMaster.NETCore.Plugins;

namespace Rift.Runtime.Modules;

public class ModuleInstance(string dllFile)
{
    private IModule?      _instance;
    private PluginLoader? _loader;

    internal bool Init()
    {
        var loader = PluginLoader.CreateFromAssemblyFile(dllFile, config =>
        {
            config.PreferSharedTypes = true;
            config.IsUnloadable      = true;

            /*
             * WARNING: 因为我们的实现里有脚本系统，而脚本系统现阶段只允许加载FromFile的Assembly文件，
             * 换句话说，不能加载内存里的Assembly，会导致Assembly.Location为空。
             * 而InteractiveAssemblyLoader加载Assembly会看Assembly.Location，且会找Runtime中加载的
             * Assembly。
             *
             * 无论如何，下述两个选项绝对不能打开，否则无法在插件处扩展脚本API！
             */
            config.EnableHotReload = false;
            config.LoadInMemory    = false;
        });

        var asm = loader.LoadDefaultAssembly();
        var module = asm.GetTypes().FirstOrDefault(t => typeof(IModule).IsAssignableFrom(t) && !t.IsAbstract) ??
                     throw new BadImageFormatException("IModule is not implemented.");

        if (Activator.CreateInstance(module) is not IModule mod)
        {
            loader.Dispose();
            return false;
        }

        _loader   = loader;
        _instance = mod;
        return true;
    }

    internal bool Load() => _instance?.OnLoad() ?? false;

    internal void AllLoaded() => _instance?.OnAllLoaded();

    internal void Unload()
    {
        if (_loader is null || _instance is null)
        {
            return;
        }

        _loader.Dispose();
        _loader = null;

        _instance.OnUnload();
        _instance = null;
    }
}