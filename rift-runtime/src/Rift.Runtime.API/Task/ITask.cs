namespace Rift.Runtime.API.Task;

public interface ITask
{
    public string Name        { get; }
    public string PackageName { get; }
    public Action? Action      { get; }

    void RegisterAction(Action action);
}