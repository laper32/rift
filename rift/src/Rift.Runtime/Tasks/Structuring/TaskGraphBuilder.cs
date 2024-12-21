using Rift.Runtime.Tasks.Fundamental;

namespace Rift.Runtime.Tasks.Structuring;

internal class TaskGraphBuilder
{
    internal static TaskGraph Build(IReadOnlyCollection<RiftTask> tasks)
    {
        var graph = new TaskGraph();

        foreach (var task in tasks)
        {
            graph.Add(task.Name);
        }

        foreach (var task in tasks)
        {
            foreach (var dependency in task.Dependencies)
            {
                if (!graph.Exists(dependency.Name))
                {
                    if (dependency.IsRequired)
                    {
                        throw new InvalidOperationException(
                            $"Task `{task.Name}` requires task `{dependency.Name}` but does not exist");
                    }
                }
                else
                {
                    graph.Connect(dependency.Name, task.Name);
                }
            }
        }

        return graph;
    }
}