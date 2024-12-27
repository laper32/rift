using Rift.Runtime.Workspace.Fundamental;

namespace Rift.Runtime.Workspace.Graph;

public class PackageGraphBuilder
{
    internal static PackageGraph Build(IReadOnlyCollection<PackageInstance> packages)
    {
        var graph = new PackageGraph();

        foreach (var package in packages)
        {
            //var node = new PackageGraphNode(package.Name, package
            //graph.Add(package);
        }

        return graph;
    }
    //internal static PackageGraph Build(IReadOnlyCollection<PackageInstance> packages)
    //{
    //    var graph = new PackageGraph();



    //    foreach (var package in packages)
    //    {
    //        foreach (var (key, value) in package.Dependencies)
    //        {
    //            Console.WriteLine($"{package.Name} => {key}");
    //        }
    //    }

    //    return graph;
    //}

    //internal static PackageGraph Build(IReadOnlyCollection<RiftTask> tasks)
    //{
    //    var graph = new PackageGraph();

    //    foreach (var task in tasks)
    //    {
    //        graph.Add(task.Name);
    //    }

    //    foreach (var task in tasks)
    //    {
    //        foreach (var dependency in task.Dependencies)
    //        {
    //            if (!graph.Exists(dependency.Name))
    //            {
    //                if (dependency.IsRequired)
    //                {
    //                    throw new InvalidOperationException(
    //                        $"Task `{task.Name}` requires task `{dependency.Name}` but does not exist");
    //                }
    //            }
    //            else
    //            {
    //                graph.Connect(dependency.Name, task.Name);
    //            }
    //        }
    //    }

    //    return graph;
    //}
}