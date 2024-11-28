using System.CommandLine;
using Rift.Runtime.Abstractions.Commands;
using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Commands;

internal interface ICommandManagerInternal : ICommandManager, IInitializable;

internal sealed class CommandManager : ICommandManagerInternal
{
    internal static  CommandManager Instance { get; private set; } = null!;
    private readonly RootCommand    _command;

    public CommandManager(InterfaceBridge bridge)
    {
        _command = new RootCommand("Rift, a cross-platform build system");
        Instance = this;
    }

    public void ExecuteCommand(string[] args)
    {
        _command.Invoke(args);
    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {
    }

    public void Something()
    {
        _command.AddCommand(new Command("a"));
    }
}