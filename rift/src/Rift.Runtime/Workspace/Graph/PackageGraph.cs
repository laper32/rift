using System.Diagnostics;
using Rift.Runtime.Workspace.Fundamental;

namespace Rift.Runtime.Workspace.Graph;

public class PackageGraph
{
    private readonly HashSet<PackageGraphEdge> _edges = [];

    private readonly HashSet<PackageGraphNode> _nodes = [];

    public IReadOnlyCollection<PackageGraphNode> Nodes => _nodes;
    public IReadOnlyCollection<PackageGraphEdge> Edges => _edges;

    public void Add(PackageGraphNode node)
    {
        ArgumentNullException.ThrowIfNull(node, nameof(node));

        if (Exists(node))
        {
            throw new ArgumentException($"Node \"{node.Name}\" (Version: {node.Version}) already exists");
        }

        _nodes.Add(node);
    }

    public bool Exists(PackageGraphNode node)
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

    public PackageGraphNode? Find(string name, string version)
    {
        return _nodes.FirstOrDefault(x => x.Equals(name, version));
    }

    public PackageGraphNode? Find(PackageReference reference)
    {
        return _nodes.FirstOrDefault(x => x.Equals(reference.Name, reference.Version));
    }

    public void Connect(PackageGraphNode start, PackageGraphNode end)
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

        _edges.Add(new PackageGraphEdge(start, end));
    }

    public PackageGraphNode GetRootNode()
    {
        foreach (var node in _nodes.Where(node => node.IsRoot()))
        {
            return node;
        }

        throw new UnreachableException();
    }

    public void Traverse(Action<PackageGraphNode> action)
    {
        var root = GetRootNode();
    }

    private void Traverse(
        PackageGraphNode node,
        Action<PackageGraphNode> action,
        ISet<PackageGraphNode>? visited = null)
    {
        visited ??= new HashSet<PackageGraphNode>();
        if (visited.Add(node))
        {
            var incoming = _edges
                .Where(x => x.End.Equals(node))
                .Select(x => x.Start);
            foreach (var child in incoming)
            {
                Traverse(node, action, visited);
            }
        }
        else if (visited.Any(x => x.Equals(node)))
        {
            throw new ArgumentException("Graph contains circular references.");
        }
        /*
                 visited ??= new HashSet<PackageGraphNode>();
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
         */
    }

    public IEnumerable<PackageGraphNode> Traverse(PackageGraphNode package)
    {
        if (!Exists(package))
        {
            return [];
        }

        var result = new List<PackageGraphNode>();
        Traverse(package, result);
        return result;
    }

    private void Traverse(
        PackageGraphNode node,
        ICollection<PackageGraphNode> result,
        ISet<PackageGraphNode>?  visited = null)
    {
        visited ??= new HashSet<PackageGraphNode>();
        if (visited.Add(node))
        {
            //                 var incoming = _edges.Where(x => x.End.Equals(node, StringComparison.OrdinalIgnoreCase)).Select(x => x.Start);

            var incoming = _edges.Where(x => x.End.Equals(node)).Select(x => x.Start);
            foreach (var child in incoming)
            {
                Traverse(child, result, visited);
            }
            result.Add(node);
        }
        else if (!result.Any(x => x.Equals(node)))
        {
            throw new ArgumentException("Graph contains circular references.");
        }
    }
}