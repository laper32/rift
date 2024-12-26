using System.Reflection;
using System.Runtime.Loader;
using Rift.Runtime.Plugins.Fundamental;

namespace Rift.Runtime.Plugins.Loader;

internal class PluginLoadContext : PluginAssemblyContext
{
    private readonly AssemblyDependencyResolver? _resolver;
    private readonly AssemblyLoadContext         _sharedContext;
    public readonly  PluginIdentity              Identity;

    public PluginLoadContext(PluginIdentity identity, AssemblyLoadContext sharedContext)
    {
        _sharedContext = sharedContext;
        Identity       = identity;
        var entryPath = Identity.EntryPath;
        if (string.IsNullOrEmpty(entryPath))
        {
            Entry = null;
            return;
        }

        _resolver = new AssemblyDependencyResolver(entryPath);
        var asmName = AssemblyName.GetAssemblyName(entryPath);
        if (_sharedContext.Assemblies.FirstOrDefault(x => x.GetName().Name == asmName.Name) is { } asm)
        {
            Entry = asm;
            return;
        }

        Entry = LoadFromAssemblyPath(entryPath);
    }

    public Assembly? Entry { get; }

    protected override Assembly? Load(AssemblyName assemblyName)
    {
        var ret = _sharedContext
            .Assemblies
            .FirstOrDefault(x => x.GetName().Name == assemblyName.Name);
        if (ret != null)
        {
            return ret;
        }

        var path = _resolver?.ResolveAssemblyToPath(assemblyName);
        return path == null ? null : LoadFromAssemblyPath(path);
    }
}