using System.Reflection;
using System.Runtime.Loader;

namespace Rift.Runtime.Modules.Loader;

internal class ModuleLoadContext : ModuleAssemblyContext
{
    private readonly AssemblyDependencyResolver? _resolver;
    private readonly AssemblyLoadContext         _sharedContext;

    public Assembly? Entry { get; }

    public ModuleLoadContext(string entryPath, AssemblyLoadContext sharedContext)
    {
        ArgumentNullException.ThrowIfNull(entryPath);
        if (string.IsNullOrEmpty(entryPath))
        {
            throw new ArgumentNullException(entryPath, nameof(entryPath));
        }

        _sharedContext = sharedContext;

        _resolver = new AssemblyDependencyResolver(entryPath);
        var asmName = AssemblyName.GetAssemblyName(entryPath);
        if (_sharedContext.Assemblies.FirstOrDefault(x => x.GetName().Name == asmName.Name) is { } asm)
        {
            Entry = asm;
        }
    }

    protected override Assembly? Load(AssemblyName assemblyName)
    {
        var ret = _sharedContext.Assemblies.FirstOrDefault(x => x.GetName().Name == assemblyName.Name);
        if (ret != null)
        {
            return ret;
        }

        var path = _resolver?.ResolveAssemblyToPath(assemblyName);
        return path == null ? null : LoadFromAssemblyPath(path);
    }
}

