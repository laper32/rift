using System.Diagnostics;

namespace Rift.Runtime.Tasks;

internal class TaskExecutor(TaskScheduler scheduler)
{
    internal TaskReport ExecuteTasks(IReadOnlyList<RiftTask> tasks)
    {
        var report = new TaskReport();
        var sw     = new Stopwatch();
        while (scheduler.TryDequeue(out var value))
        {
            if (tasks.First(x => x.Name.Equals(value.Name, StringComparison.OrdinalIgnoreCase)) is not { } task)
            {
                continue;
            }
            ExecuteTask(task, value.Context, sw, report);
        }

        return report;
    }

    internal void ExecuteTask(RiftTask task, TaskContext ctx, Stopwatch sw, TaskReport report)
    {
        ExecuteTaskAsync(task, ctx, sw, report).GetAwaiter().GetResult();
    }

    internal async Task ExecuteTaskAsync(RiftTask task, TaskContext ctx, Stopwatch sw, TaskReport report)
    {
        sw.Restart();

        Exception? taskException = null;

        try
        {
            await task.Invoke(ctx).ConfigureAwait(false);
        }
        catch (Exception e)
        {
            taskException = e;
            if (task.ErrorHandler is not null)
            {
                await HandleErrorAsync(task.ErrorHandler, taskException, ctx);
            }
            else
            {
                throw;
            }
        }

        if (taskException is null)
        {
            report.Add(task.Name, sw.Elapsed);
        }
        else
        {
            report.AddFailed(task.Name, sw.Elapsed);
        }

    }

    private static async Task HandleErrorAsync(Func<Exception, TaskContext, Task> errorHandler, Exception exception, TaskContext context)
    {
        await errorHandler(exception, context);
    }
}