// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Reflection;
using System.Runtime.Loader;

namespace Rift.Runtime.Plugin;

internal class InstanceContext : AssemblyLoadContext
{
    public InstanceContext() : base(true)
    {
        Resolving += (_, args) => AppDomain.CurrentDomain.GetAssemblies().Where(x => x.GetName().Name == args.Name).FirstOrDefault();
    }
}

internal class PluginContext : InstanceContext
{
    private readonly AssemblyDependencyResolver _resolver;
    private readonly AssemblyLoadContext _sharedContext;
    public Assembly Entry { get; }
    public string EntryPath { get; }
    public string PluginPath => Directory.GetParent(EntryPath)!.FullName;
    public PluginContext(string entryPath, AssemblyLoadContext sharedContext)
    {
        _sharedContext = sharedContext;
        EntryPath = entryPath;
        _resolver = new AssemblyDependencyResolver(entryPath);
        var asmName = AssemblyName.GetAssemblyName(EntryPath);
        if (_sharedContext.Assemblies.Where(x => x.GetName().Name == asmName.Name).FirstOrDefault() is { } asm)
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
            .Where(x => x.GetName().Name == assemblyName.Name)
            .FirstOrDefault();
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