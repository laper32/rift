using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

public class TaskReportRecipe(
    string                  taskName,
    string                  skippedMessage,
    TimeSpan                duration,
    RiftTaskExecutionStatus executionStatus)
    : ITaskReportRecipe
{
    public string                  TaskName        { get; } = taskName;
    public string                  SkippedMessage  { get; } = skippedMessage;
    public TimeSpan                Duration        { get; } = duration;
    public RiftTaskExecutionStatus ExecutionStatus { get; } = executionStatus;

    public TaskReportRecipe(string taskName, string skippedMessage, TimeSpan duration)
        : this(taskName, skippedMessage, duration, RiftTaskExecutionStatus.Executed)
    {
    }
}