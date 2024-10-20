// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.System;
using Rift.Runtime.Fundamental;
using System.Diagnostics.CodeAnalysis;

namespace Rift.Runtime.System;

// ReSharper disable UnusedMemberInSuper.Global

internal interface IShareSystemInternal : IShareSystem, IInitializable
{
    IEnumerable<ISharable> GetPluginInterfaces(IPlugin plugin);

    void RemoveInterface(ISharable @interface);
}

internal class ShareSystem(InterfaceBridge bridge) : IShareSystemInternal
{
    public bool Init()
    {
        Console.WriteLine("ShareSystem init.");
        return true;
    }

    public void Shutdown()
    {
        Console.WriteLine("ShareSystem shutdown.");
    }

    public void AddInterface(ISharable @interface, IPlugin plugin)
    {
        var name = @interface.InterfaceName;
        if (_interfaces.Any(x => x.Instance.InterfaceName.Equals(name)))
        {
            throw new InvalidOperationException($"Interface with name {name} already exists.");
        }
        _interfaces.Add(new ShareableInfo(@interface, plugin));
    }

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

    public bool HasInterface<T>() where T : class, ISharable
    {
        return _interfaces.Where(instance => instance.Instance.GetType()
                .GetInterfaces()
                .Any(@interface => @interface == typeof(T)))
            .Any();
    }

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

    private record ShareableInfo(ISharable Instance, IPlugin Plugin);

    /// <summary>
    /// 有些插件可以有多个接口, 没必要搞得这么复杂, 一个List完事.
    /// </summary>
    private readonly List<ShareableInfo> _interfaces = [];
}