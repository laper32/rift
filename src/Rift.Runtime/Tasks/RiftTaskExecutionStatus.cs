// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

/// <summary>
///     The execution status of <see cref="IRiftTask" />
/// </summary>
public enum RiftTaskExecutionStatus
{
    /// <summary>
    ///     The task was executed.
    /// </summary>
    Executed,

    /// <summary>
    ///     The task delegated execution.
    /// </summary>
    Delegated,

    /// <summary>
    ///     The task was skipped.
    /// </summary>
    Skipped,

    /// <summary>
    ///     The task failed.
    /// </summary>
    Failed
}