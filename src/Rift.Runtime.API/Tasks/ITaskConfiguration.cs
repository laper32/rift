﻿namespace Rift.Runtime.API.Tasks;

/// <summary>
/// 注册任务时的配置
/// </summary>
public interface ITaskConfiguration
{
    public ITaskConfiguration SetDeferException(bool value);
}