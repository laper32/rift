using System.CommandLine;
using Rift.Runtime.Tasks;

namespace Rift.Runtime.Commands;

public sealed class CommandManager
{
    private RootCommand _command = null!;
    private bool        _initialized;

    public CommandManager()
    {
        Instance = this;
    }

    public static CommandManager Instance { get; private set; } = null!;

    public static void ExecuteCommand(string[] args)
    {
        if (!Instance._initialized) BuildCli();
        Instance.Invoke(args);
    }

    internal static bool Init()
    {
        return true;
    }

    internal static void Shutdown()
    {
    }

    private static void BuildCli()
    {
        if (Instance._initialized) return;
        var pendingCommands = TaskManager.Instance.GetMarkedAsCommandTasks();

        var entries = UserCommand.Build(pendingCommands);
        Instance._command = UserCommand.BuildCli(entries);

        Instance._initialized = true;
    }

    private void Invoke(string[] args)
    {
        _command.Invoke(args);
    }
}