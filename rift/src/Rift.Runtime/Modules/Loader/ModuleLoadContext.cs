using System.Reflection;
using System.Runtime.Loader;
using Rift.Runtime.Modules.Fundamental;

namespace Rift.Runtime.Modules.Loader;

internal class ModuleLoadContext : ModuleAssemblyContext
{
    private readonly AssemblyDependencyResolver _resolver;
    private readonly AssemblyLoadContext        _sharedContext;

    public ModuleLoadContext(ModuleIdentity identity, AssemblyLoadContext sharedContext)
    {
        ArgumentNullException.ThrowIfNull(identity, nameof(identity));
        ArgumentNullException.ThrowIfNull(sharedContext, nameof(sharedContext));

        _sharedContext = sharedContext;
        Identity       = identity;
        var entryPath = Identity.EntryPath;
        if (string.IsNullOrEmpty(entryPath))
        {
            throw new BadImageFormatException("Entry path is null");
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

    public ModuleIdentity Identity { get; init; }
    public Assembly       Entry    { get; init; }

    protected override Assembly? Load(AssemblyName assemblyName)
    {
        Console.WriteLine($"Loading: {assemblyName}");
        var ret = _sharedContext
            .Assemblies
            .FirstOrDefault(x => x.GetName().Name == assemblyName.Name);
        if (ret != null)
        {
            return ret;
        }

        var baseAsm = AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName() == assemblyName);
        Console.WriteLine($"baseAsm: {baseAsm?.FullName}");
        if (baseAsm != null)
        {
            return baseAsm;
        }

        var path = _resolver.ResolveAssemblyToPath(assemblyName);
        if (path is null)
        {
            return null;
        }

        var asm = LoadFromAssemblyPath(path);
        return asm;
    }
}