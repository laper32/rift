using System.CommandLine;
using System.Text.Json;
using Rift.Runtime.Abstractions.Tasks;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Tasks;

namespace Rift.Runtime.Commands;

internal class UserCommand
{
    public static UserCommandEntry Build(IEnumerable<string> commandTasks)
    {
        var root = new UserCommandEntry("etc");
        foreach (var task in commandTasks)
        {
            var parts = task.Split('.');
            parts = parts[1..];
            var currentNode = root;
            foreach (var part in parts)
            {
                // Find or create the child node
                var childNode = currentNode.Children.FirstOrDefault(c => c.Name == part) ?? currentNode.AddChild(part);
                // Move to the child node
                currentNode = childNode;
            }
            // Add the task to the leaf node
            currentNode.TaskName = task;
        }
        return root;
    }

    public static void PrintTree(UserCommandEntry node, string indent = "")
    {
        // Print the current node's name
        Console.WriteLine($"{indent}{node.Name}");
        // Print the tasks for the leaf node
        if (!string.IsNullOrEmpty(node.TaskName))
        {
            Console.WriteLine($"{indent}  - {node.TaskName}");
        }

        // Recursively print the children
        foreach (var child in node.Children)
        {
            PrintTree(child, indent + "  ");
        }
    }

    public static RootCommand BuildCli(UserCommandEntry entry, InterfaceBridge bridge)
    {
        var root = new RootCommand("Rift, a cross-platform build system");
        BuildCliImpl(root, entry, bridge);
        return root;
    }

    private static void BuildCliImpl(Command cmd, UserCommandEntry entry, InterfaceBridge bridge)
    {
        foreach (var child in entry.Children)
        {
            var newCmd = new Command(child.Name);
            cmd.AddCommand(newCmd);
            if (TaskManager.Instance.FindTask(child.TaskName) is not RiftTask task)
            {
                throw new TaskNotFoundException($"{child.TaskName} does not found in registered tasks.");
            }

            if (child.Children.Count > 0)
            {
                if (task.HasAction || task.HasDelayedAction)
                {
                    throw new Exception("Task with children cannot have actions.");
                }
            }

            newCmd.Description = task.Description;
            if (task.HasAction)
            {
                Console.WriteLine(
                    $"Command Name: {cmd.Name}, Task: {task.Name}, actions count: {task.Actions.Count}");

                newCmd.SetHandler(() =>
                {
                    Console.WriteLine($"Running task: {task.Name}...");
                    TaskManager.Instance.RunTask(task.Name);

                    //Task.Run(async () =>
                    //{
                    //    await task.Invoke(new TaskContext(bridge));
                    //}).Wait();
                });
            }
            
            //Console.WriteLine($"ChildNode: {child.Name}, Children count: {child.Children.Count}");
            BuildCliImpl(newCmd, child, bridge);
        }
    }
}