using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Task;

namespace Rift.Runtime.Task;

internal class Task(string packageName, TaskManifest manifest) : ITask
{
    public TaskManifest Manifest { get; init; } = manifest;

    public string Name => Manifest.Name;

    public string PackageName { get; init; } = packageName;

    public Action? Action { get; private set; }

    public bool IsCommand { get; set; }

    public List<ITask>  RunTasks { get; init; } = [];

    /// <summary>
    /// 只存名字（aka: id)
    /// </summary>
    public List<string> SubTasks { get; init; } = [];

    public void RegisterAction(Action action)
    {
        if (Action is not null)
        {
            return;
        }

        Action = action;
    }
}
