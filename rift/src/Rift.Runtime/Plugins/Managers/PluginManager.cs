﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Plugins.Abstractions;
using Rift.Runtime.Plugins.Fundamental;
using Rift.Runtime.Plugins.Loader;
using Rift.Runtime.Workspace.Fundamental;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Plugins.Managers;

public sealed class PluginManager
{
    internal delegate void DelegatePluginUnload(PluginInstance instance);

    private static PluginManager _instance = null!;

    private readonly PluginIdentities               _identities;
    private readonly List<PluginInstance>           _instances           = [];
    private readonly List<PluginIdentity>           _pendingLoadPlugins  = [];
    private readonly List<PluginLoadContext>        _pluginContexts      = [];
    private readonly List<PluginSharedAssemblyInfo> _sharedAssemblyInfos = [];
    private          PluginAssemblyContext?         _sharedContext;

    public PluginManager()
    {
        _identities    = new PluginIdentities();
        _sharedContext = new PluginAssemblyContext();
        _instance      = this;
    }

    internal static event DelegatePluginUnload? PluginUnload;

    internal static bool Init()
    {
        return GetInstance().InitInternal();
    }

    private bool InitInternal()
    {
        return true;
    }

    internal static void Shutdown()
    {
        GetInstance().ShutdownInternal();
    }

    private void ShutdownInternal()
    {
        UnloadPlugins();
    }

    /// <summary>
    ///     N.B. 插件系统这里和很多地方不一样的是：我们需要支持没有dll的情况（即：这个插件只有二进制文件，或者只有配置文件）<br />
    ///     所以必须有一个中间层给插件做Identity。
    /// </summary>
    internal static void NotifyLoadPlugins()
    {
        GetInstance().NotifyLoadPluginsInternal();
    }

    internal void NotifyLoadPluginsInternal()
    {
        var declarators = WorkspaceManager.CollectPluginsForLoad();
        foreach (var declarator in declarators)
        {
            _identities.Add(declarator);
        }

        _pendingLoadPlugins.AddRange(_identities.GetSortedIdentities());

        BootPlugins();
        LoadPlugins();
    }

    private void BootPlugins()
    {
        AddSharedAssemblies();
        LoadSharedContext();
        LoadPluginContext();
        ActivateInstance();
        CleanupTemporaries();
    }

    private void LoadPlugins()
    {
        foreach (var instance in _instances.Where(instance => instance.Init()))
        {
            instance.Load();
        }

        foreach (var instance in _instances)
        {
            instance.PostLoad();
        }
    }


    private void AddSharedAssemblies()
    {
        var sharedAssemblies = new Dictionary<
            string,                        // 需要共享的asm文件名
            List<PluginSharedAssemblyInfo> // 整个插件系统中出现的二进制文件信息
        >();
        _pendingLoadPlugins.ForEach(x =>
        {
            var identityPluginSharedAssemblyInfos = x.PluginSharedAssemblyInfos;

            foreach (var (name, info) in identityPluginSharedAssemblyInfos)
            {
                if (!sharedAssemblies.TryGetValue(name, out var value))
                {
                    value = [];
                    sharedAssemblies.Add(name, value);
                }

                sharedAssemblies[name].Add(info);
            }
        });

        // 排序规则
        // 1. ProductVersion: Major > Minor > Build
        // 2. 上一次的文件修改日期, 以最近的为基准
        // 3. 如果前两个还是不行, 那么就只拿列表中的第一个
        // N.B. 大多数情况下, 版本号只有Major.Minor.Build, 第四个是Revision, FileVersionInfo里管它叫Private, 用法千奇百怪,
        // 但无论如何前三个是我所见到的各种打版本号规则里, 最为通用的.
        //
        // 如果Major.Minor.Build都相同, 且此时我们发现同名的dll有多个, 那么就有如下情况:
        //  1. 真的是一模一样的dll, 发行时为了保险加了多份.
        //  2. 开发阶段: 因为开发阶段往往是下一个版本, 但还没定稿, 这时候沿用原来的版本号也是很正常的.
        // 如果是情况1, 那么一切好说, 重点是情况2: 这时候只能看dll的修改时间来判断, 如果这个dll修改日期越近, 那自然就能说明
        // 这个dll是最新版, 反之, 版本号比较老.
        //
        // 此外, 如果说, Major.Minor.Build一样, 这里设置两个同名dll, 叫A.1和A.2, A.1比A.2要旧, 但他们的版本号一致, 存在一种情况,
        // A.1的修改日期比A.2要新(这种一般是因为有些工具写入文件选择直接创建新文件导致的)
        // 我不认为我们需要处理这种情况, 因为既然有这种情况发生, 说明用户已经知晓了这种行为可能会带来的后果. 而且就我所知的大多数情况下, 
        // 版本号相同而实际执行出现问题, 这种是属于插件作者的问题, 而不是框架的问题.
        // 记住, 我们只能保证提供的是最新版, 但我们不能保证内部的执行细节.
        foreach (var (_, value) in sharedAssemblies)
        {
            // 这里主要是为了尽量兼顾开发和运行.
            // 开发阶段往往会出现这么个情况: 东西没写完, 版本号不递进, 但是每次都在编译, 每次都有产出.
            // 那么这时候版本号一定是一样的, 但区别在于写入日期.
            // 而为了保证插件的正常运行, 我们自然会选择最近写入的dll
            // 如果是正式上线的话, 你不应该走到第四步, 也就是看写入日期这一步.
            // 如果真走到了这一步, 一般情况下，你不如想想你的整个项目组织是不是出问题了.
            // (非一般情况如：CS2更新，需要紧急修复游戏更新导致的炸服，根本不可能留时间走完CI的全套流程)
            // 此外, 开发阶段应当也是基于版本号的. 因为经常有backport的情况出现, 重点还是为了开发方便就是了.
            var first = value
                .OrderByDescending(x => x.Info.ProductMajorPart)
                .ThenByDescending(x => x.Info.ProductMinorPart)
                .ThenByDescending(x => x.Info.ProductBuildPart)
                .ThenByDescending(x => x.LastWriteDate)
                .First();

            _sharedAssemblyInfos.Add(first);
        }
    }

    private void LoadSharedContext()
    {
        foreach (var info in _sharedAssemblyInfos)
        {
            _sharedContext!.LoadFromAssemblyPath(info.Path);
        }
    }

    private void LoadPluginContext()
    {
        foreach (var identity in _pendingLoadPlugins)
        {
            _pluginContexts.Add(new PluginLoadContext(identity, _sharedContext!));
        }
    }

    private void ActivateInstance()
    {
        foreach (var context in _pluginContexts)
        {
            GetPluginInstances().Add(new PluginInstance(context));
        }
    }

    private void CleanupTemporaries()
    {
        _pendingLoadPlugins.Clear();
    }

    private void UnloadPlugins()
    {
        foreach (var instance in GetPluginInstances())
        {
            PluginUnload?.Invoke(instance);
            instance.Unload(true);
        }

        GetPluginInstances().Clear();
        _sharedContext!.Unload();
        _sharedContext = null;
        foreach (var context in _pluginContexts)
        {
            context.Unload();
        }

        _pluginContexts.Clear();
    }

    internal static bool AddDependencyForPlugin(PackageReference declarator)
    {
        return GetInstance().AddDependencyForPluginInternal(declarator);
    }

    private bool AddDependencyForPluginInternal(PackageReference declarator)
    {
        return GetPluginIdentities().AddDependencyForPlugin(declarator);
    }

    internal static bool AddDependencyForPlugin(IEnumerable<PackageReference> declarators)
    {
        return GetInstance().AddDependencyForPluginInternal(declarators);
    }

    private bool AddDependencyForPluginInternal(IEnumerable<PackageReference> declarators)
    {
        return GetPluginIdentities().AddDependencyForPlugin(declarators);
    }

    public static PluginRuntimeInfo? GetPluginRuntimeInfo(RiftPlugin plugin)
    {
        if (GetInstance().GetPluginInstances().Find(x => x.Instance == plugin) is not { } instance)
        {
            return null;
        }

        var identity = instance.Identity;
        var manifest = identity.GetManifest();
        return new PluginRuntimeInfo(manifest.Name, manifest.Version, instance.Status);
    }

    private List<PluginInstance> GetPluginInstances() => _instances;

    internal static PluginManager GetInstance() => _instance;

    private PluginIdentities GetPluginIdentities() => _identities;
}