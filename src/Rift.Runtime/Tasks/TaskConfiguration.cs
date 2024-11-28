using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

internal class TaskConfiguration(RiftTask task) : ITaskConfiguration
{
    public ITaskConfiguration SetDeferException(bool value)
    {
        task.DeferExceptions = value;
        return this;
    }

    public ITaskConfiguration SetErrorHandler(Func<Exception, ITaskContext, Task> predicate)
    {
        task.SetErrorHandler(predicate);
        return this;
    }

    public ITaskConfiguration SetIsCommand(bool value)
    {
        task.IsCommand = value;
        return this;
    }

    public ITaskConfiguration AddAction(Action<ITaskContext> action)
    {
        task.Actions.Add((context =>
        {
            action(context);
            return Task.CompletedTask;
        }));
        return this;
    }

    public ITaskConfiguration SetDescription(string description)
    {
        task.Description = description;
        return this;
    }
}