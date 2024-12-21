using System.Reflection;
using System.Runtime.Loader;
using Rift.Runtime.Modules.Fundamental;

namespace Rift.Runtime.Modules.Loader;

internal class ModuleLoadContext : ModuleAssemblyContext
{
    private readonly AssemblyDependencyResolver _resolver;
    private readonly AssemblyLoadContext        _sharedContext;

    public ModuleIdentity Identity { get; init; }
    public Assembly       Entry    { get; init; }

    public ModuleLoadContext(ModuleIdentity identity, AssemblyLoadContext sharedContext)
    {
        ArgumentNullException.ThrowIfNull(identity, nameof(identity));
        ArgumentNullException.ThrowIfNull(sharedContext, nameof(sharedContext));

        _sharedContext = sharedContext;
        Identity = identity;
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
        Console.WriteLine($"EntryPath: {entryPath}, Entry: {Entry is null}");
        Entry = LoadFromAssemblyPath(entryPath);
        Console.WriteLine($"EntryPath: {entryPath}, Entry: {Entry is null}");

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
        return path == null ? null : LoadFromAssemblyPath(path);
    }
}

