using Rift.Runtime.API.Fundamental;
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
        return true;
    }

    public void Shutdown()
    {
    }
}