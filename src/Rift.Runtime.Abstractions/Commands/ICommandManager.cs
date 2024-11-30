namespace Rift.Runtime.Abstractions.Commands;

public interface ICommandManager
{
    void ExecuteCommand(string[] args);
}