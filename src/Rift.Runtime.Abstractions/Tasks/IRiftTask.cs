// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Abstractions.Tasks;

public interface IRiftTask
{
    /// <summary>
    /// 名字
    /// </summary>
    string Name        { get; }

    /// <summary>
    /// 描述
    /// </summary>
    string Description { get; }

    /// <summary>
    /// 如果需要执行这个task，需要哪些task提前执行？
    /// </summary>
    IReadOnlyList<IDependentTask> Dependencies { get; }

    /// <summary>
    /// 这个task会被哪些task依赖？
    /// </summary>
    IReadOnlyList<IDependentTask> Dependents { get; }
}