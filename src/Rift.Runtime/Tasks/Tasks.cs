using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

// ReSharper disable UnusedMember.Global
public class Tasks
{
    public static void Register(string name, Action<ITaskConfiguration> configure)
    {
        var task          = (RiftTask)TaskManager.Instance.RegisterTask(name);
        var configuration = new TaskConfiguration();
        configure(configuration);
        task.AddConfiguration(configuration);
    }
}