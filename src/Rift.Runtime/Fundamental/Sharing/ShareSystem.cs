// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;
using Rift.Runtime.Plugin;
using System.Diagnostics.CodeAnalysis;

namespace Rift.Runtime.Fundamental.Sharing;


public sealed class ShareSystem : IInitializable
{
    public ShareSystem()
    {
        _logger  = Runtime.Instance.Logger.CreateLogger<ShareSystem>();
        Instance = this;
    }

    private readonly ILogger<ShareSystem> _logger;
    internal static ShareSystem Instance = null!;

    private record ShareableInfo(ISharable Instance, IPlugin Plugin);

    /// <summary>
    /// 有些插件可以有多个接口, 没必要搞得这么复杂, 一个List完事.
    /// </summary>
    private readonly List<ShareableInfo> _interfaces = [];

    /// <summary>
    /// 添加接口
    /// </summary>
    /// <param name="interface">接口类</param>
    /// <param name="plugin">插件实例</param>
    public void AddInterface(ISharable @interface, IPlugin plugin)
    {
        var name = @interface.InterfaceName;
        if (_interfaces.Any(x => x.Instance.InterfaceName.Equals(name)))
        {
            throw new InvalidOperationException($"Interface with name {name} already exists.");
        }
        _interfaces.Add(new ShareableInfo(@interface, plugin));
    }

    /// <summary>
    /// 获取接口
    /// </summary>
    /// <typeparam name="T">接口类</typeparam>
    /// <param name="version">版本号</param>
    /// <returns>期望的接口, 为空说明没有</returns>
    public T? GetInterface<T>(uint version) where T : class, ISharable
    {
        foreach (var instance in _interfaces
                     .Where(instance =>
                         instance
                             .Instance
                             .GetType()
                             .GetInterfaces()
                             .Any(@interface => @interface == typeof(T))
                     )
                )
        {
            // 传入进来的版本号, 和ShareSystem里面存着的版本号的对应关系
            //  InterfaceVersion        ParamVersion
            //          1                     1         => OK
            //          1                     2         => No
            //          2                     1         => OK
            // => InterfaceVersion的版本号必须>=传入进来的版本号.

            if (instance.Instance.InterfaceVersion < version)
            {
                return null;
            }

            return (T?)instance.Instance;
        }

        return null;
    }

    /// <summary>
    /// 查询是否有接口 <br />
    /// <remarks>
    /// 该函数只用于确认是否有, 不涉及到具体的版本号, 如果你想获取, 或知道是否有某个特定版本号的接口, 请用 <br />
    /// - <see cref="GetInterface{T}"/> <br />
    /// - <see cref="GetRequiredInterface{T}"/> <br />
    /// - <see cref="TryGetInterface{T}"/>
    /// </remarks>
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <returns></returns>
    public bool HasInterface<T>() where T : class, ISharable
    {
        return _interfaces
            .Any(instance => instance.Instance.GetType()
                .GetInterfaces()
                .Any(@interface => @interface == typeof(T)));
    }

    /// <summary>
    /// 获取必须的接口, 如果不存在则抛出异常.
    /// </summary>
    /// <typeparam name="T">你期望的类型s</typeparam>
    /// <param name="version">接口版本号</param>
    /// <returns>期望的接口</returns>
    /// <exception cref="NotImplementedException">
    ///     如果查不到这个接口就说明这个接口没有实现. <br />
    ///     无论你怎么操作, 此时均有可能会直接崩溃服务器. <br />
    ///     使用该函数之前请确保你的运行环境确实存在该接口!
    /// </exception>
    public T GetRequiredInterface<T>(uint version) where T : class, ISharable
    {
        foreach (var instance in _interfaces
                     .Where(instance =>
                         instance
                             .Instance
                             .GetType()
                             .GetInterfaces()
                             .Any(@interface => @interface == typeof(T))
                     )
                )
        {
            if (instance.Instance.InterfaceVersion > version)
            {
                throw new NotImplementedException($"Interface <{typeof(T).Name}> version is lower.");
            }

            return (T)instance.Instance;
        }

        throw new NotImplementedException($"Interface <{typeof(T).Name}> not found.");
    }

    /// <summary>
    /// 根据接口名和版本号, 尝试获取某个接口
    /// </summary>
    /// <typeparam name="T">期望的接口类型</typeparam>
    /// <param name="name">接口名</param>
    /// <param name="version">接口版本号</param>
    /// <param name="ret">返回值</param>
    /// <returns>True说明有, 否则没有</returns>
    public bool TryGetInterface<T>(string name, uint version, [MaybeNullWhen(returnValue: false)] out T ret) where T : class, ISharable
    {
        foreach (var instance in _interfaces.Where(instance => instance.Instance.InterfaceName == name && instance.Instance.InterfaceVersion >= version))
        {
            ret = (T)instance.Instance;
            return true;
        }
        ret = null;
        return false;
    }

    public IEnumerable<ISharable> GetPluginInterfaces(IPlugin plugin)
        => _interfaces
            .Where(x => x.Plugin == plugin)
            .Select(x => x.Instance);

    public void RemoveInterface(ISharable @interface)
    {
        var instance = _interfaces.FirstOrDefault(x => x.Instance == @interface);
        if (instance != null)
        {
            _interfaces.Remove(instance);
        }
    }

    public bool Init()
    {
        PluginManager.Instance.PluginUnload += OnPluginUnload;
        return true;
    }

    public void Shutdown()
    {
        PluginManager.Instance.PluginUnload -= OnPluginUnload;
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
}