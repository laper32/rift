// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.CommandLine;
using Rift.Runtime.Commands.Cli;
using Rift.Runtime.Tasks.Managers;

namespace Rift.Runtime.Commands.Managers;

/// <summary>
/// Manages the execution and initialization of CLI commands.
/// </summary>
public sealed class CommandManager
{
    private RootCommand _command = null!;
    private bool _initialized;

    /// <summary>
    /// Initializes a new instance of the <see cref="CommandManager"/> class.
    /// </summary>
    public CommandManager()
    {
        Instance = this;
    }

    /// <summary>
    /// Gets the singleton instance of the <see cref="CommandManager"/> class.
    /// </summary>
    internal static CommandManager Instance { get; private set; } = null!;

    /// <summary>
    /// Executes the specified command arguments.
    /// </summary>
    /// <param name="args">The command arguments to execute.</param>
    public static void ExecuteCommand(string[] args)
    {
        if (!Instance._initialized)
        {
            BuildCli();
        }

        Instance.Invoke(args);
    }

    /// <summary>
    /// Initializes the command manager.
    /// </summary>
    /// <returns>True if initialization is successful; otherwise, false.</returns>
    internal static bool Init()
    {
        return true;
    }

    /// <summary>
    /// Shuts down the command manager.
    /// </summary>
    internal static void Shutdown()
    {
    }

    /// <summary>
    /// Builds the CLI commands.
    /// </summary>
    private static void BuildCli()
    {
        if (Instance._initialized)
        {
            return;
        }

        var pendingCommands = TaskManager.GetMarkedAsCommandTasks();

        var entries = UserCommand.Build(pendingCommands);
        Instance._command = UserCommand.BuildCli(entries);

        Instance._initialized = true;
    }

    /// <summary>
    /// Invokes the specified command arguments.
    /// </summary>
    /// <param name="args">The command arguments to invoke.</param>
    private void Invoke(string[] args)
    {
        _command.Invoke(args);
    }
}
