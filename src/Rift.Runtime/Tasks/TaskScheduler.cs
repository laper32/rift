using System.Collections.Concurrent;
using System.Diagnostics.CodeAnalysis;

namespace Rift.Runtime.Tasks;

internal record ScheduledTask(string Name, TaskContext Context);

internal class TaskScheduler
{
    private readonly ConcurrentQueue<ScheduledTask> _scheduledTasks = [];

    public void Enqueue(string name)
    {
        Enqueue(name, new TaskContext());
    }

    public void Enqueue(string name, TaskContext context)
    {
        _scheduledTasks.Enqueue(new ScheduledTask(name, context));
    }

    public bool TryDequeue([MaybeNullWhen(false)] out ScheduledTask value)
    {
        if (_scheduledTasks.TryDequeue(out var result))
        {
            value = result;
            return true;
        }
        
        value = null;
        return false;
    }
}