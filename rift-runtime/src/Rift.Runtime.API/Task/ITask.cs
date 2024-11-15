namespace Rift.Runtime.API.Task;

public interface ITask
{
    public string       Name        { get; }
    public bool         IsCommand   { get; }
    public List<string> SubTasks    { get; }
    public List<string> RunTasks    { get; }
    public string       PackageName { get; }
    public Action?      Action      { get; }

    void RegisterAction(Action action);
}