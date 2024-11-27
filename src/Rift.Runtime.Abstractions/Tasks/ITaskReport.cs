namespace Rift.Runtime.Abstractions.Tasks;

public interface ITaskReport
{
    string                  TaskName        { get; }
    string                  SkippedMessage  { get; }
    TimeSpan                Duration        { get; }
    RiftTaskExecutionStatus ExecutionStatus { get; }
}

public interface ITaskReportRecipe
{
    void Add(ITaskReport report);
}