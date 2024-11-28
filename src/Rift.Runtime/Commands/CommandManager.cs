using Rift.Runtime.Abstractions.Commands;
using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Commands;

internal interface ICommandManagerInternal : ICommandManager, IInitializable;

internal sealed class CommandManager : ICommandManagerInternal
{
    private readonly List<UserCommand> _commands = [];
    internal static  CommandManager    Instance { get; private set; } = null!;

    public CommandManager(InterfaceBridge bridge)
    {
        Instance = this;
    }

    public void ExecuteCommand(string[] args)
    {
        throw new NotImplementedException();
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
        
    }
}