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
    internal bool IsCommand { get; set; }

    [JsonIgnore]
    internal List<Func<TaskContext, Task>> Actions { get; init; } = [];

    [JsonIgnore]
    internal Queue<Action<TaskContext>> DelayedActions { get; init; } = [];

    [JsonIgnore]
    internal Func<Exception, TaskContext, Task>? ErrorHandler { get; private set; }

    internal List<ITaskArgument> Arguments { get; init; } = [];
    internal List<ITaskOption>   Options   { get; init; } = [];

    internal bool DeferExceptions { get; set; }

    internal bool                 HasAction => Actions.Count > 0;
    internal bool                 HasDelayedAction => DelayedActions.Count > 0;
    internal string               Name { get; } = name ?? throw new ArgumentNullException(name, nameof(name));
    internal string               Description { get; set; } = string.Empty;
    public   List<IDependentTask> Dependencies { get; init; } = [];

    internal void SetErrorHandler(Func<Exception, TaskContext, Task> predicate)
    {
        ArgumentNullException.ThrowIfNull(predicate, nameof(predicate));

        ErrorHandler = predicate;
    }

    /// <summary>
    ///     Executes the task using the specified context.
    /// </summary>
    /// <param name="context"> The context. </param>
    /// <returns> Returned Task. </returns>
    internal async Task Invoke(TaskContext context)
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