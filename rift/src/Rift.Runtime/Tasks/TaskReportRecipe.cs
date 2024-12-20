// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

public class TaskReportRecipe(
    string                  taskName,
    string                  skippedMessage,
    TimeSpan                duration,
    RiftTaskExecutionStatus executionStatus)
{
    public TaskReportRecipe(string taskName, string skippedMessage, TimeSpan duration)
        : this(taskName, skippedMessage, duration, RiftTaskExecutionStatus.Executed)
    {
    }

    public string                  TaskName        { get; } = taskName;
    public string                  SkippedMessage  { get; } = skippedMessage;
    public TimeSpan                Duration        { get; } = duration;
    public RiftTaskExecutionStatus ExecutionStatus { get; } = executionStatus;
}