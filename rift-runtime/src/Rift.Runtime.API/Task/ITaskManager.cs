namespace Rift.Runtime.API.Task;

public interface ITaskManager
{
    public static ITaskManager Instance { get; protected set; } = null!;
}