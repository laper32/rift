namespace Rift.Runtime.Abstractions.Tasks;

/// <summary>
/// 注册任务时的配置
/// </summary>
public interface ITaskConfiguration
{
    ITaskConfiguration SetDeferException(bool value);

    ITaskConfiguration SetErrorHandler(Func<Exception, ITaskContext, Task> predicate);

    ITaskConfiguration SetIsCommand(bool value);
    ITaskConfiguration AddAction(Action<ITaskContext> action);
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