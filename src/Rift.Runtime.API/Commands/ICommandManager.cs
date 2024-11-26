namespace Rift.Runtime.API.Commands;

public interface ICommandManager
{
    void ExecuteCommand(string[] args);
}