// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.CommandLine;
using Rift.Runtime.Commands.Data;
using Rift.Runtime.Tasks.Data;
using Rift.Runtime.Tasks.Exceptions;
using Rift.Runtime.Tasks.Fundamental;
using Rift.Runtime.Tasks.Managers;

namespace Rift.Runtime.Commands.Cli;

internal class UserCommand
{
    public static UserCommandEntry Build(IEnumerable<string> commandTasks)
    {
        var root = new UserCommandEntry("rift");
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

    public static RootCommand BuildCli(UserCommandEntry entry)
    {
        var root = new RootCommand("Rift, a cross-platform build system");

        root.SetHandler(() => { root.Invoke("--help"); });
        BuildCliImpl(root, entry);
        return root;
    }

    private static void BuildCliImpl(Command cmd, UserCommandEntry entry)
    {
        foreach (var child in entry.Children)
        {
            var newCmd = new Command(child.Name);

            if (TaskManager.FindTask(child.TaskName) is not { } task)
            {
                throw new TaskNotFoundException($"{child.TaskName} does not found in registered tasks.");
            }

            task.Options.ForEach(x => { newCmd.AddOption(x.Value); });

            task.Arguments.ForEach(x => { newCmd.AddArgument(x.Value); });

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
                newCmd.SetHandler(ctx =>
                {
                    var commandArgs = new CommandArguments();

                    var options = newCmd
                        .Options
                        .ToDictionary(
                            opt => opt.Name,
                            opt => ctx.ParseResult.GetValueForOption(opt)
                        );
                    var args = newCmd
                        .Arguments
                        .ToDictionary(
                            args => args.Name,
                            args => ctx.ParseResult.GetValueForArgument(args)
                        );
                    commandArgs.AddArguments(args);
                    commandArgs.AddOptions(options);
                    TaskManager.ScheduleTask(task.Name, new TaskContext
                    {
                        Data = new TaskData(commandArgs)
                    });
                });
            }

            cmd.AddCommand(newCmd);
            BuildCliImpl(newCmd, child);
        }
    }
}