// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics.CodeAnalysis;
using Rift.Runtime.Plugins;

namespace Rift.Runtime.Interfaces;


public sealed class InterfaceManager
{
    private record InterfaceInformation(IInterface Instance, IPlugin Plugin);
    private readonly List<InterfaceInformation> _interfaces = [];
    private static InterfaceManager _instance = null!;

    public InterfaceManager()
    {
        _instance = this;
    }

    public static void AddInterface<T>(T @interface, IPlugin plugin) where T : class, IInterface =>
        _instance.AddInterfaceInternal(@interface, plugin);

    private void AddInterfaceInternal<T>(T @interface, IPlugin plugin) where T:class, IInterface
    {
        var version = @interface.InterfaceVersion;
        if (_interfaces.Any(x =>  x.Instance.InterfaceVersion.Equals(version)))
        {
            throw new InvalidOperationException($"Interface `{typeof(T).Name}(version: {version})` already exists");
        }
        _interfaces.Add(new InterfaceInformation(@interface, plugin));
    }

    /// <summary>
    /// 获取接口
    /// </summary>
    /// <typeparam name="T">接口类</typeparam>
    /// <param name="version">版本号</param>
    /// <returns>期望的接口, 为空说明没有</returns>
    public static T? GetInterface<T>(uint version) where T : class, IInterface =>
        _instance.GetInterfaceInternal<T>(version);

    private T? GetInterfaceInternal<T>(uint version) where T : class, IInterface
    {
        // 传入进来的版本号, 和InterfaceManager里面存着的版本号的对应关系
        //  InterfaceVersion        ParamVersion        ResultVersion
        //          1                     1                   1         => OK
        //          1                     2                   /         => No
        //          2                     1                   2         => OK
        // => InterfaceVersion的版本号必须>=传入进来的版本号.

        // 如果内部有多版实现, 则是如下情况:
        //  InterfaceVersion        ParamVersion        ResultVersion
        //     [1, 2, 3]                 1                    1         => OK
        //    [12, 13, 14]              15                    /         => No
        //    [12, 13, 14]              11                   14         => OK
        // 换句话说, 如果内部有多版本实现, 且同时匹配到了确切的版本, 则返回对应的版本
        // 如果是低于现有实现的版本, 则直接返回最新版
        // 否则, 挂掉

        var interfaces =
            _interfaces
                .Where(instance =>
                    instance
                        .Instance
                        .GetType()
                        .GetInterfaces()
                        .Any(x => x == typeof(T))
                ).ToArray();

        var interfaceVersions = interfaces
            .Select(x => x.Instance.InterfaceVersion)
            .OrderByDescending(x => x)
            .ToArray();
        var oldestInterfaceVersion = interfaceVersions.Min();
        var latestInterfaceVersion = interfaceVersions.Max();
        if (interfaceVersions.Contains(version))
        {
            var @interface = interfaces.First(x => x.Instance.InterfaceVersion == version);
            return (T) @interface.Instance;
        }

        // version 11, runtime minimum: 12 => choose 14
        if (version < oldestInterfaceVersion)
        {
            var @interface = interfaces.First(x => x.Instance.InterfaceVersion == latestInterfaceVersion);
            return (T) @interface.Instance;
        }

        // version 15, runtime provided: 14 => No
        if (version > latestInterfaceVersion)
        {
            return null;
        }

        return null;
    }

    /// <summary>
    /// 查询是否有接口 <br />
    /// <remarks>
    /// 该函数只用于确认是否有, 不涉及到具体的版本号, 如果你想获取, 或知道是否有某个特定版本号的接口, 请用 <br />
    /// - <see cref="GetInterface{T}"/> <br />
    /// - <see cref="GetRequiredInterface{T}"/> <br />
    /// - <see cref="TryGetInterface{T}"/> <br/>
    /// - <see cref="HasInterface{T}(uint)"/> <br/>
    /// </remarks>
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <returns></returns>
    public static bool HasInterface<T>() where T : class, IInterface => _instance.HasInterfaceInternal<T>();

    private bool HasInterfaceInternal<T>() where T : class, IInterface
    {
        return _interfaces
            .Any(instance => instance.Instance.GetType()
                .GetInterfaces()
                .Any(@interface => @interface == typeof(T)));
    }

    public static bool HasInterface<T>(uint version) where T : class, IInterface =>
        _instance.HasInterfaceInternal<T>(version);

    private bool HasInterfaceInternal<T>(uint version) where T : class, IInterface =>
        GetInterfaceInternal<T>(version) is not null;


    /// <summary>
    /// 获取必须的接口, 如果不存在则抛出异常.
    /// </summary>
    /// <typeparam name="T">你期望的类型</typeparam>
    /// <param name="version">接口版本号</param>
    /// <returns>期望的接口</returns>
    /// <exception cref="NotImplementedException">
    ///     如果查不到这个接口就说明这个接口没有实现. <br />
    ///     你需要自行处理异常. <br />
    ///     使用该函数之前请确保你的运行环境确实存在该接口!
    /// </exception>
    public static T GetRequiredInterface<T>(uint version) where T : class, IInterface =>
        _instance.GetRequiredInterfaceInternal<T>(version);

    private T GetRequiredInterfaceInternal<T>(uint version) where T : class, IInterface
    {
        // 传入进来的版本号, 和InterfaceManager里面存着的版本号的对应关系
        //  InterfaceVersion        ParamVersion        ResultVersion
        //          1                     1                   1         => OK
        //          1                     2                   /         => No
        //          2                     1                   2         => OK
        // => InterfaceVersion的版本号必须>=传入进来的版本号.

        // 如果内部有多版实现, 则是如下情况:
        //  InterfaceVersion        ParamVersion        ResultVersion
        //     [1, 2, 3]                 1                    1         => OK
        //    [12, 13, 14]              15                    /         => No
        //    [12, 13, 14]              11                   14         => OK
        // 换句话说, 如果内部有多版本实现, 且同时匹配到了确切的版本, 则返回对应的版本
        // 如果是低于现有实现的版本, 则直接返回最新版
        // 否则, 挂掉

        var interfaces =
            _interfaces
                .Where(instance =>
                    instance
                        .Instance
                        .GetType()
                        .GetInterfaces()
                        .Any(x => x == typeof(T))
                ).ToArray();

        var interfaceVersions = interfaces
            .Select(x => x.Instance.InterfaceVersion)
            .OrderByDescending(x => x)
            .ToArray();
        var oldestInterfaceVersion = interfaceVersions.Min();
        var latestInterfaceVersion = interfaceVersions.Max();
        if (interfaceVersions.Contains(version))
        {
            var @interface = interfaces.First(x => x.Instance.InterfaceVersion == version);
            return (T) @interface.Instance;
        }

        // version 11, runtime minimum: 12 => choose 12
        if (version < oldestInterfaceVersion)
        {
            var @interface = interfaces.First(x => x.Instance.InterfaceVersion == latestInterfaceVersion);
            return (T) @interface.Instance;
        }

        // version 15, runtime provided: 14 => No
        if (version > latestInterfaceVersion)
        {
            throw new NotSupportedException(
                $"Interface <{typeof(T).Name}> version requested ({version}) is not provided in runtime (Available versions: [{string.Join(", ", interfaceVersions)}]).");
        }

        throw new EntryPointNotFoundException($"Interface <{typeof(T).Name}> not found.");
    }

    /// <summary>
    /// 根据接口名和版本号, 尝试获取某个接口
    /// </summary>
    /// <typeparam name="T">期望的接口类型</typeparam>
    /// <param name="version">接口版本号</param>
    /// <param name="ret">返回值</param>
    /// <returns>True说明有, 否则没有</returns>
    public static bool TryGetInterface<T>(uint version, [MaybeNullWhen(returnValue: false)] out T ret)
        where T : class, IInterface => _instance.TryGetInterfaceInternal(version, out ret);

    private bool TryGetInterfaceInternal<T>(uint version, [MaybeNullWhen(returnValue: false)] out T ret)
        where T : class, IInterface
    {
        if (GetInterfaceInternal<T>(version) is { } result)
        {
            ret = result;
            return true;
        }

        ret = null;
        return false;
    }

    internal static IEnumerable<IInterface> GetPluginInterfaces(IPlugin plugin) =>
        _instance.GetPluginInterfacesInternal(plugin);

    private IEnumerable<IInterface> GetPluginInterfacesInternal(IPlugin plugin)
        => _interfaces
            .Where(x => x.Plugin == plugin)
            .Select(x => x.Instance);

    internal static void RemoveInterface<T>(T @interface) where T : class, IInterface =>
        _instance.RemoveInterfaceInternal(@interface);

    private void RemoveInterfaceInternal<T>(T @interface) where T : class, IInterface
    {
        var instance = _interfaces.FirstOrDefault(x => x.Instance == @interface);
        if (instance != null)
        {
            _interfaces.Remove(instance);
        }
    }

    internal static bool Init()
    {
        //PluginManager.Instance.PluginUnload += Instance.OnPluginUnload;
        return true;
    }

    internal static void Shutdown()
    {
        //PluginManager.Instance.PluginUnload -= Instance.OnPluginUnload;
    }

    //private void OnPluginUnload(PluginInstance instance)
    //{
    //    if (instance.Instance is not { } internalInstance)
    //    {
    //        return;
    //    }
    //    foreach (var @interface in GetPluginInterfaces(internalInstance).ToArray())
    //    {
    //        RemoveInterface(@interface);
    //    }
    //}
}