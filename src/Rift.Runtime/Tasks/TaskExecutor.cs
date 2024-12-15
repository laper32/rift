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
        await task.Invoke(ctx).ConfigureAwait(false);

        report.Add(new TaskReportRecipe(task.Name, "", sw.Elapsed));
        Console.WriteLine($"Running task: {task.Name}, time elapsed: {sw.Elapsed}");
    }
}