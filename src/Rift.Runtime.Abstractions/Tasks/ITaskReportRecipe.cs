namespace Rift.Runtime.Abstractions.Tasks;

public interface ITaskReportRecipe
{
    string                  TaskName        { get; }
    string                  SkippedMessage  { get; }
    TimeSpan                Duration        { get; }
    RiftTaskExecutionStatus ExecutionStatus { get; }
}
