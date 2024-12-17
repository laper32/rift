// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics.CodeAnalysis;
using System.Text.Json;
using Rift.Runtime.Plugins;

namespace Rift.Runtime.Interfaces;

/// <summary>
///     用于处理插件之间的接口共享
/// </summary>
public sealed class InterfaceManager
{
    private static   InterfaceManager           _instance   = null!;
    private readonly List<InterfaceInformation> _interfaces = [];

    public InterfaceManager()
    {
        _instance = this;
    }

    internal static bool Init()
    {
        PluginManager.PluginUnload += _instance.OnPluginUnload;
        return true;
    }

    internal static void Shutdown()
    {
        PluginManager.PluginUnload -= _instance.OnPluginUnload;
    }

    /// <summary>
    ///     注册接口
    /// </summary>
    /// <typeparam name="T"> 继承自<see cref="IInterface" />的接口类型 </typeparam>
    /// <param name="interface"> 接口实例 </param>
    /// <param name="plugin"> 对应的插件 </param>
    /// <exception cref="InterfaceAlreadyExistsException"> 如果接口已经存在则抛出异常 </exception>
    public static void AddInterface<T>(T @interface, IPlugin plugin) where T : class, IInterface
    {
        var version = @interface.InterfaceVersion;
        if (_instance._interfaces.Any(x => x.Instance.InterfaceVersion.Equals(version)))
        {
            throw new InterfaceAlreadyExistsException(
                $"Interface `{typeof(T).Name}(version: {version})` already exists");
        }

        _instance._interfaces.Add(new InterfaceInformation(@interface, plugin));
    }

    /// <summary>
    ///     获取接口
    /// </summary>
    /// <typeparam name="T"> 接口类 </typeparam>
    /// <param name="version"> 版本号 </param>
    /// <returns> 期望的接口, 为空说明没有 </returns>
    public static T? GetInterface<T>(uint version) where T : class, IInterface
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
        //    [12, 13, 14]              11                    /         => No
        // 换句话说, 只看runtime内提供的版本号，找不到直接挂掉
        // N.B.: 未来可以扩展成如果找不到就去nuget上下，连nuget都找不到就直接挂掉

        var interfaces =
            _instance._interfaces
                .Where(instance =>
                    instance
                        .Instance
                        .GetType()
                        .GetInterfaces()
                        .Any(x => x == typeof(T))
                ).ToArray();

        var interfaceVersions = interfaces
            .Select(x => x.Instance.InterfaceVersion)
            .ToArray();

        if (!interfaceVersions.Contains(version))
        {
            return null;
        }

        var @interface = interfaces.First(x => x.Instance.InterfaceVersion == version);
        return (T)@interface.Instance;
    }

    /// <summary>
    ///     查询是否有接口 <br />
    ///     <remarks>
    ///         该函数只用于确认是否有, 不涉及到具体的版本号, 如果你想获取, 或知道是否有某个特定版本号的接口, 请用 <br />
    ///         - <see cref="GetInterface{T}" /> <br />
    ///         - <see cref="GetRequiredInterface{T}" /> <br />
    ///         - <see cref="TryGetInterface{T}" /> <br />
    ///         - <see cref="HasInterface{T}(uint)" /> <br />
    ///     </remarks>
    /// </summary>
    /// <typeparam name="T"> 继承自<see cref="IInterface" />的接口 </typeparam>
    /// <returns> True if interface exists, false otherwise. </returns>
    public static bool HasInterface<T>() where T : class, IInterface
    {
        return _instance._interfaces
            .Any(instance => instance.Instance.GetType()
                .GetInterfaces()
                .Any(@interface => @interface == typeof(T)));
    }

    /// <summary>
    ///     根据传入的类型和版本号查询是否有该接口。 <br />
    ///     <remarks>
    ///         如果你需要进行进一步的操作，用<br />
    ///         - <see cref="GetInterface{T}" /> <br />
    ///         - <see cref="GetRequiredInterface{T}" /> <br />
    ///         - <see cref="TryGetInterface{T}" /> <br />
    ///     </remarks>
    /// </summary>
    /// <typeparam name="T"> 继承自<see cref="IInterface" />的接口 </typeparam>
    /// <param name="version"> 接口版本号 </param>
    /// <returns> True if interface exists, false otherwise. </returns>
    public static bool HasInterface<T>(uint version) where T : class, IInterface
    {
        return GetInterface<T>(version) is not null;
    }

    /// <summary>
    ///     获取必须的接口, 如果不存在则抛出异常.
    /// </summary>
    /// <typeparam name="T"> 你期望的类型 </typeparam>
    /// <param name="version"> 接口版本号 </param>
    /// <returns> 期望的接口 </returns>
    /// <exception cref="InterfaceNotFoundException">
    ///     如果查不到这个接口就说明这个接口没有实现. <br />
    ///     你需要自行处理异常. <br />
    ///     使用该函数之前请确保你的运行环境确实存在该接口!
    /// </exception>
    public static T GetRequiredInterface<T>(uint version) where T : class, IInterface
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
        //    [12, 13, 14]              11                    /         => No
        // 换句话说, 只看runtime内提供的版本号，找不到直接挂掉
        // N.B.: 未来可以扩展成如果找不到就去nuget上下，连nuget都找不到就直接挂掉

        var interfaces =
            _instance._interfaces
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
        if (interfaceVersions.Contains(version))
        {
            var @interface = interfaces.First(x => x.Instance.InterfaceVersion == version);
            return (T)@interface.Instance;
        }

        // version 11, runtime minimum: 12 => Found but version not provided.
        // version 15, runtime provided: 14 => Found but version not provided.
        // 版本区间：[oldestInterfaceVersion, latestInterfaceVersion]
        if (version < interfaceVersions.Min() || version > interfaceVersions.Max())
        {
            throw new InterfaceNotFoundException(
                $"Interface <{typeof(T).Name}> version requested ({version}) is not provided in runtime (Available versions: [{string.Join(", ", interfaceVersions)}])."
            );
        }

        throw new InterfaceNotFoundException($"Interface <{typeof(T).Name}> not found.");
    }

    /// <summary>
    ///     根据接口名和版本号, 尝试获取某个接口
    /// </summary>
    /// <typeparam name="T"> 期望的接口类型 </typeparam>
    /// <param name="version"> 接口版本号 </param>
    /// <param name="ret"> 返回值 </param>
    /// <returns> True说明有, 否则没有 </returns>
    public static bool TryGetInterface<T>(uint version, [MaybeNullWhen(false)] out T ret)
        where T : class, IInterface
    {
        if (GetInterface<T>(version) is { } result)
        {
            ret = result;
            return true;
        }

        ret = null;
        return false;
    }

    internal static void RemoveInterface<T>(T @interface) where T : class, IInterface
    {
        var instance = _instance._interfaces.FirstOrDefault(x => x.Instance == @interface);
        if (instance != null)
        {
            _instance._interfaces.Remove(instance);
        }
    }

    internal static void DumpPluginInterfaces()
    {
        _instance._interfaces.ForEach(Console.WriteLine);
    }

    private IEnumerable<IInterface> GetPluginInterfaces(IPlugin plugin)
    {
        return _interfaces
            .Where(x => x.Plugin == plugin)
            .Select(x => x.Instance);
    }

    private void OnPluginUnload(PluginInstance instance)
    {
        if (instance.Instance is not { } internalInstance)
        {
            return;
        }

        foreach (var @interface in GetPluginInterfaces(internalInstance).ToArray())
        {
            RemoveInterface(@interface);
        }
    }

    private record InterfaceInformation(IInterface Instance, IPlugin Plugin)
    {
        public override string ToString()
        {
            return $"InterfaceInformation {{ Instance = {Instance.GetType()}, Plugin = {Plugin.GetType()} }}";
        }
    }
}