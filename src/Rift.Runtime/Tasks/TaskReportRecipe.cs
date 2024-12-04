namespace Rift.Runtime.Tasks;

public interface ITaskReportRecipe
{
    string                  TaskName        { get; }
    string                  SkippedMessage  { get; }
    TimeSpan                Duration        { get; }
    RiftTaskExecutionStatus ExecutionStatus { get; }
}

public class TaskReportRecipe(
    string                  taskName,
    string                  skippedMessage,
    TimeSpan                duration,
    RiftTaskExecutionStatus executionStatus)
    : ITaskReportRecipe
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