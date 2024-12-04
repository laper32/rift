namespace Rift.Runtime.Tasks;

internal class TaskContext : ITaskContext
{
    public ITaskArguments Arguments { get; }
}

public interface ITaskContext
{
    ITaskArguments Arguments { get; }
}