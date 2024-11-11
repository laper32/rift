using System.Reflection;
using System.Runtime.Loader;

namespace Rift.Runtime.Plugin;

internal class PluginInstanceContext : AssemblyLoadContext
{
    public PluginInstanceContext() : base(true)
    {
        Resolving += (_, args) => AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName().Name == args.Name);
    }
}

internal class PluginContext : PluginInstanceContext
{
    private readonly AssemblyDependencyResolver _resolver;
    private readonly AssemblyLoadContext        _sharedContext;
    public           Assembly                   Entry      { get; }
    public           string                     EntryPath  { get; }
    //public           string                     PluginPath => (Directory.GetParent(EntryPath)!.FullName);

    public PluginContext(string entryPath, AssemblyLoadContext sharedContext)
    {
        _sharedContext = sharedContext;
        EntryPath      = entryPath;
        _resolver      = new AssemblyDependencyResolver(entryPath);
        var asmName = AssemblyName.GetAssemblyName(EntryPath);
        if (_sharedContext.Assemblies.FirstOrDefault(x => x.GetName().Name == asmName.Name) is { } asm)
        {
            Entry = asm;
            return;
        }

        using var fs = new FileStream(entryPath, FileMode.Open);
        Entry = LoadFromStream(fs);
    }

    protected override Assembly? Load(AssemblyName assemblyName)
    {
        var ret = _sharedContext
            .Assemblies
            .FirstOrDefault(x => x.GetName().Name == assemblyName.Name);
        if (ret != null)
        {
            return ret;
        }

        var path = _resolver.ResolveAssemblyToPath(assemblyName);
        if (path == null)
        {
            return null;
        }

        var fs = new FileStream(path, FileMode.Open);
        return LoadFromStream(fs);
    }
}
