using Rift.Runtime.Commands;
using Rift.Runtime.Tasks;
using Xunit.Abstractions;

namespace Rift.Runtime.Tests;

public class UnitTest1(ITestOutputHelper testOutputHelper) : RuntimeSetup
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
        var commands = TaskManager.Instance.GetMarkedAsCommandTasks();
        var entries = UserCommand.Build(commands);

        //CommandManager.Instance.ExecuteCommand(Environment.GetCommandLineArgs());
    }
}