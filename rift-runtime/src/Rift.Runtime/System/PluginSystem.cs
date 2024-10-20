// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.System;
using System.Reflection;
using System.Runtime.Loader;
using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Enums;
using Rift.Runtime.Fundamental;
using static Rift.Runtime.System.IPluginSystemInternal;
using System.Diagnostics;

namespace Rift.Runtime.System;

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


internal interface IPluginSystemInternal : IPluginSystem, IInitializable
{
    public delegate void DelegatePluginUnload(PluginInstance instance);

    public event DelegatePluginUnload PluginUnload;

    public ILogger<PluginSystem> Logger { get; }

    void SetPluginState(RiftPlugin instance, PluginStatus state, Exception? error = null);
}

internal class PluginSystem(InterfaceBridge bridge) : IPluginSystemInternal
{
    private record PluginSharedAssemblyInfo(string Path, FileVersionInfo Info, DateTime LastWriteDate);

    private InstanceContext? _sharedContext;

    private readonly List<string> _pendingPluginEntryPaths = [];
    private readonly List<string> _pendingPluginPaths = [];
    private readonly List<string> _pendingPluginSharedAssemblyPaths = [];
    private readonly List<PluginInstance> _instances = [];
    private readonly List<PluginContext> _pluginContexts = [];
    private readonly List<PluginSharedAssemblyInfo> _sharedAssemblyInfos = [];
    private readonly List<string> _modifiedPlugins = [];

    // Key: Shared Assembly Name, Value: SharedAssemblyInfo
    private readonly Dictionary<string, List<PluginSharedAssemblyInfo>> _pendingPluginSharedAssemblyInfos = [];

    public ILogger<PluginSystem> Logger { get; init; } = bridge.Logger.CreateLogger<PluginSystem>();

    public event DelegatePluginUnload? PluginUnload;

    public void SetFailState(RiftPlugin instance, Exception reason)
    {
        throw new NotImplementedException();
    }

    public IPluginRuntimeInfo? GetPluginRuntimeInfo(RiftPlugin instance)
    {
        throw new NotImplementedException();
    }

    public IEnumerable<IPluginRuntimeInfo> GetAllPluginRuntimeInfo()
    {
        throw new NotImplementedException();
    }

    public bool Init()
    {
        Console.WriteLine("PluginSystem initialized.");
        return true;
    }

    public void Shutdown()
    {
        Console.WriteLine("Shutting down PluginSystem");
    }

    public void SetPluginState(RiftPlugin instance, PluginStatus state, Exception? error = null)
    {
        throw new NotImplementedException();
    }
}