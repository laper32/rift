using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Task;

namespace Rift.Runtime.Task;

internal interface ITaskManagerInternal : ITaskManager
{

}

internal class TaskManager : ITaskManagerInternal, IInitializable
{
    public TaskManager()
    {
        ITaskManager.Instance = this;
    }

    public bool Init()
    {
        IScriptSystem.Instance.AddNamespaces(["Rift.Runtime.Task"]);
        return true;
    }

    public void Shutdown()
    {
    }
}