namespace Rift.Runtime.Tasks;

internal class TaskGraph
{
    private readonly List<string>        _nodes = [];
    private readonly List<TaskGraphEdge> _edges = [];

    public IReadOnlyList<string>        Nodes => _nodes;
    public IReadOnlyList<TaskGraphEdge> Edges => _edges;

    public void Add(string node)
    {
        ArgumentNullException.ThrowIfNull(node, nameof(node));
        if (Exists(node))
        {
            throw new ArgumentException($"Node \"{node}\" already exists");
        }

        _nodes.Add(node);
    }

    public void Connect(string start, string end)
    {
        if (start.Equals(end, StringComparison.OrdinalIgnoreCase))
        {
            throw new ArgumentException("Reflexive edges in graph are not allowed.");
        }

        if (_edges.Any(x =>
                    x.Start.Equals(end, StringComparison.OrdinalIgnoreCase) &&
                    x.End.Equals(start, StringComparison.OrdinalIgnoreCase))
            )
        {
            var firstBadEdge = _edges.First(x =>
                x.Start.Equals(end, StringComparison.OrdinalIgnoreCase) &&
                x.End.Equals(start, StringComparison.OrdinalIgnoreCase)
            );
            throw new ArgumentException(
                $"Unidirectional edges in graph are not allowed.{Environment.NewLine}\"{firstBadEdge.Start}\" and \"{firstBadEdge.End}\" cannot depend on each other.");
        }
        if (_edges.Any(x => x.Start.Equals(start, StringComparison.OrdinalIgnoreCase)
                            && x.End.Equals(end, StringComparison.OrdinalIgnoreCase)))
        {
            return;
        }
        if (_nodes.All(x => !x.Equals(start, StringComparison.OrdinalIgnoreCase)))
        {
            _nodes.Add(start);
        }
        if (_nodes.All(x => !x.Equals(end, StringComparison.OrdinalIgnoreCase)))
        {
            _nodes.Add(end);
        }
        _edges.Add(new TaskGraphEdge(start, end));
    }

    public bool Exists(string node) => _nodes.Any(x => x.Equals(node, StringComparison.OrdinalIgnoreCase));


    public IEnumerable<string> Traverse(string task)
    {
        if (!Exists(task))
        {
            return [];
        }

        var result = new List<string>();
        Traverse(task, result);
        return result;
    }

    private void Traverse(string node, ICollection<string> result, ISet<string>? visited = null)
    {
        visited = visited ?? new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        if (!visited.Contains(node))
        {
            visited.Add(node);
            var incoming = _edges.Where(x => x.End.Equals(node, StringComparison.OrdinalIgnoreCase)).Select(x => x.Start);
            foreach (var child in incoming)
            {
                Traverse(child, result, visited);
            }
        } else if (!result.Any(x => x.Equals(node, StringComparison.OrdinalIgnoreCase)))
        {
            throw new ArgumentException("Graph contains circular references.");
        }
    }
}