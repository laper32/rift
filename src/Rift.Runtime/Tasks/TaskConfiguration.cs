
namespace Rift.Runtime.Tasks;


/// <summary>
/// 注册任务时的配置
/// </summary>
public interface ITaskConfiguration
{
    ITaskConfiguration SetDeferException(bool value);

    ITaskConfiguration SetErrorHandler(Func<Exception, ITaskContext, Task> predicate);

    ITaskConfiguration SetIsCommand(bool              value);
    ITaskConfiguration AddAction(Action<ITaskContext> action);
    ITaskConfiguration SetDescription(string          description);
}

public static class TaskConfigurationExtensions
{
    public static ITaskConfiguration AddAction(this ITaskConfiguration configuration, Action action)
    {
        return configuration.AddAction(_ =>
        {
            action();
        });
    }
}

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