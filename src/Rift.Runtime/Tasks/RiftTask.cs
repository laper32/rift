// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using System.Text.Json.Serialization;

namespace Rift.Runtime.Tasks;

public class RiftTask(string name)
{
    [JsonIgnore]
    private readonly List<IDependentTask> _dependencies = [];

    [JsonIgnore]
    private readonly List<IDependentTask> _dependents = [];

    public bool IsCommand { get; set; }

    [JsonIgnore]
    public List<Func<ITaskContext, Task>> Actions { get; init; } = [];

    [JsonIgnore]
    public Queue<Action<ITaskContext>> DelayedActions { get; init; } = [];

    [JsonIgnore]
    public Func<Exception, ITaskContext, Task>? ErrorHandler { get; private set; }

    public bool DeferExceptions { get; set; }

    public bool                          HasAction => Actions.Count > 0;
    public bool                          HasDelayedAction => DelayedActions.Count > 0;
    public string                        Name { get; } = name ?? throw new ArgumentNullException(name, nameof(name));
    public string                        Description { get; set; } = string.Empty;
    public IReadOnlyList<IDependentTask> Dependencies => _dependencies;
    public IReadOnlyList<IDependentTask> Dependents => _dependents;

    public void SetErrorHandler(Func<Exception, ITaskContext, Task> predicate)
    {
        ArgumentNullException.ThrowIfNull(predicate, nameof(predicate));

        ErrorHandler = predicate;
    }

    /// <summary>
    ///     Executes the task using the specified context.
    /// </summary>
    /// <param name="context"> The context. </param>
    /// <returns> Returned Task. </returns>
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

    public override string ToString()
    {
        return JsonSerializer.Serialize(this, new JsonSerializerOptions
        {
            WriteIndented = true
        });
    }
}