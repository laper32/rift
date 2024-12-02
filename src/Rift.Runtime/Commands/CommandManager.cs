using System.CommandLine;
using Rift.Runtime.Tasks;

namespace Rift.Runtime.Commands;


public sealed class CommandManager
{
    public static  CommandManager  Instance { get; private set; } = null!;
    private          RootCommand     _command     = null!;
    private          bool            _initialized;

    public CommandManager()
    {
        Instance = this;
    }

    public void ExecuteCommand(string[] args)
    {
        if (!_initialized)
        {
            BuildCli();
        }
        _command.Invoke(args);

    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {
    }

    public void BuildCli()
    {
        if (_initialized)
        {
            return;
        }
        var pendingCommands = TaskManager.Instance.GetMarkedAsCommandTasks();

        var entries = UserCommand.Build(pendingCommands);
        _command     = UserCommand.BuildCli(entries);

        _initialized = true;
    }
}