using Rift.Runtime.Commands;
using Rift.Runtime.Tasks;

namespace Rift.Runtime.Tests;

public class UnitTest1 : RuntimeSetup
{
    [Fact]
    public void Test1()
    {
        TaskManager.Instance.RegisterTask("rift.new", (config) =>
        {
            config.SetIsCommand(true);
        });
        TaskManager.Instance.RegisterTask("rift.new.classlib", config =>
        {
            config.SetIsCommand(true);
        });
        var pendingCommands = TaskManager.Instance.GetMarkedAsCommandTasks();


        var entries = UserCommand.Build(pendingCommands);
        UserCommand.PrintTree(entries);
    }
}