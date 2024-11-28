namespace Rift.Runtime.Tasks;

internal class TaskGraphEdge(string start, string end)
{
    public string Start { get; } = start;
    public string End   { get; } = end;
}