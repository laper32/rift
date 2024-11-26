// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Abstractions.Tasks;

public interface ITaskManager
{
    /// <summary>
    /// 注册一个任务 <br/>
    /// <remarks>
    /// 如果该任务已经存在，将返回已经存在的任务。 <br/>
    /// </remarks>
    /// </summary>
    /// <param name="name">任务名</param>
    /// <returns>想获取的任务</returns>
    IRiftTask RegisterTask(string name);

    /// <summary>
    /// 判断该任务是否存在
    /// </summary>
    /// <param name="name">任务名</param>
    /// <returns>想获取的任务</returns>
    bool HasTask(string name);
}