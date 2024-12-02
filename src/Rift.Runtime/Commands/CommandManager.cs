using System.CommandLine;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Commands;

public interface ICommandManager
{
    void ExecuteCommand(string[] args);
}

internal interface ICommandManagerInternal : ICommandManager, IInitializable;

internal sealed class CommandManager : ICommandManagerInternal
{
    internal static  CommandManager  Instance { get; private set; } = null!;
    private readonly InterfaceBridge _bridge;
    private          RootCommand     _command     = null!;
    private          bool            _initialized;

    public CommandManager(InterfaceBridge bridge)
    {
        _bridge  = bridge;
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
        var pendingCommands = _bridge.TaskManager.GetMarkedAsCommandTasks();

        var entries = UserCommand.Build(pendingCommands);
        _command     = UserCommand.BuildCli(entries, _bridge);
        _initialized = true;
    }
}