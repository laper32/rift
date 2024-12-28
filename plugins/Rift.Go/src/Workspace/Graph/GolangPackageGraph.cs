namespace Rift.Go.Workspace.Graph;

internal class GolangPackageGraph
{
    private readonly HashSet<GolangPackageGraphNode> _nodes = [];
    private readonly HashSet<GolangPackageGraphEdge> _edges = [];

    public IReadOnlyCollection<GolangPackageGraphNode> Nodes => _nodes;
    public IReadOnlyCollection<GolangPackageGraphEdge> Edges => _edges;

    public void Add(GolangPackageGraphNode node)
    {
        ArgumentNullException.ThrowIfNull(node, nameof(node));
   
        if (Exists(node))
        {
            throw new ArgumentException($"Node \"{node.Name}\" (Version: {node.Version}) already exists");
        }
   
        _nodes.Add(node);
    }

    public bool Exists(GolangPackageGraphNode node)
    {
        ArgumentNullException.ThrowIfNull(node, nameof(node));
   
        return _nodes.Any(x => x.Equals(node));
    }
   
    public bool Exists(string name, string version)
    {
        ArgumentNullException.ThrowIfNull(name, nameof(name));
        ArgumentNullException.ThrowIfNull(version, nameof(version));
        return _nodes.Any(x => x.Equals(name, version));
    }


    public void Connect(GolangPackageGraphNode start, GolangPackageGraphNode end)
    {
        if (start.Equals(end))
        {
            throw new ArgumentException("Reflexive edges in graph are not allowed.");
        }
   
        if (_edges.Any(x =>
                x.Start.Equals(end) &&
                x.End.Equals(start))
           )
        {
            var firstBadEdge = _edges.First(x =>
                x.Start.Equals(end) &&
                x.End.Equals(start)
            );
   
            throw new ArgumentException(
                $"Unidirectional edges in graph are not allowed.{Environment.NewLine}\"{firstBadEdge.Start}\" and \"{firstBadEdge.End}\" cannot depend on each other."
            );
        }
   
        if (_edges.Any(x =>
                x.Start.Equals(start) &&
                x.End.Equals(end))
           )
        {
            return;
        }
   
        if (_nodes.All(x => !x.Equals(start)))
        {
            _nodes.Add(start);
        }
   
        if (_nodes.All(x => !x.Equals(end)))
        {
            _nodes.Add(end);
        }
   
        _edges.Add(new GolangPackageGraphEdge(start, end));
    }

    public IEnumerable<GolangPackageGraphNode> Traverse(GolangPackageGraphNode package)
    {
        if (!Exists(package))
        {
            return [];
        }
   
        var result = new List<GolangPackageGraphNode>();
        Traverse(package, result);
        return result;
    }
   
    private void Traverse(
        GolangPackageGraphNode node,
        ICollection<GolangPackageGraphNode> result,
        ISet<GolangPackageGraphNode>?  visited = null)
    {
        visited ??= new HashSet<GolangPackageGraphNode>();
        if (visited.Add(node))
        {
            var incoming = _edges.Where(x => x.End.Equals(node)).Select(x => x.Start);
            foreach (var child in incoming)
            {
                Traverse(child, result, visited);
            }
        }
        else if (!result.Any(x => x.Equals(node)))
        {
            throw new ArgumentException("Graph contains circular references.");
        }
    }
}
