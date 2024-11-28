namespace Rift.Runtime.Commands;

public class UserCommand(string commandName, string taskName)
{
    public string CommandName { get; } = commandName;
    public string TaskName    { get; } = taskName;
}