using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

internal class RiftTask(string name) : IRiftTask
{
    public string Name { get; } = name ?? throw new ArgumentNullException(name, nameof(name));
    public string Description { get; set; } = string.Empty;
    public bool                                IsCommand      { get; set; }
    public List<IRiftDependentTask>            Dependencies   { get; init; } = [];
    public List<IRiftDependentTask>            Dependents     { get; init; } = [];
    public List<Func<ITaskContext, Task>>      Actions        { get; init; } = [];
    public Queue<Action<ITaskContext>>         DelayedActions { get; init; } = [];
    public Func<Exception, ITaskContext, Task> ErrorHandler   { get; }
    public bool DeferExceptions { get; set; }
    public ITaskConfiguration? Configuration { get; private set; }

    public void AddConfiguration(ITaskConfiguration configuration)
    {
        ArgumentNullException.ThrowIfNull(configuration, nameof(configuration));

        if (Configuration is not null)
        {
            return;
        }

        Configuration = configuration;
    }

    /// <summary>
    /// Executes the task using the specified context.
    /// </summary>
    /// <param name="context">The context.</param>
    /// <returns>Returned Task.</returns>
    public async Task Invoke(ITaskContext context)
    {
        while (DelayedActions.Count > 0)
        {
            var delayedDelegate = DelayedActions.Dequeue();
            delayedDelegate(context);
        }

        var exceptions = new List<Exception>();
        foreach (var action in Actions)
        {
            try
            {
                await action(context).ConfigureAwait(false);
            }
            catch (Exception e) when (DeferExceptions)
            {
                exceptions.Add(e);
            }
        }

        if (exceptions.Any())
        {
            if (exceptions.Count == 1)
            {
                throw exceptions.Single();
            }
            throw new AggregateException("Task failed with following exceptions", exceptions);
        }
    }
}