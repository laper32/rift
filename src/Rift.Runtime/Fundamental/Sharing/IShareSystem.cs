// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics.CodeAnalysis;
using Rift.Runtime.Plugin;

namespace Rift.Runtime.Fundamental.Sharing;

public interface IShareSystem
{
    /// <summary>
    /// 添加接口
    /// </summary>
    /// <param name="interface">接口类</param>
    /// <param name="plugin">插件实例</param>
    void AddInterface(ISharable @interface, IPlugin plugin);

    /// <summary>
    /// 获取接口
    /// </summary>
    /// <typeparam name="T">接口类</typeparam>
    /// <param name="version">版本号</param>
    /// <returns>期望的接口, 为空说明没有</returns>
    T? GetInterface<T>(uint version) where T : class, ISharable;

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
    bool HasInterface<T>() where T : class, ISharable;

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
    T GetRequiredInterface<T>(uint version) where T : class, ISharable;

    /// <summary>
    /// 根据接口名和版本号, 尝试获取某个接口
    /// </summary>
    /// <typeparam name="T">期望的接口类型</typeparam>
    /// <param name="name">接口名</param>
    /// <param name="version">接口版本号</param>
    /// <param name="ret">返回值</param>
    /// <returns>True说明有, 否则没有</returns>
    public bool TryGetInterface<T>(string name, uint version, [MaybeNullWhen(returnValue: false)] out T ret)
        where T : class, ISharable;
}
