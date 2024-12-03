using System.Collections.ObjectModel;
using Rift.Runtime.Collections.Generic;
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

    [Fact]
    public void DictionaryForEach()
    {
        var dict = new Dictionary<string, string>
        {
            {"a", "b"},
            {"c", "d"}
        };
        dict.ForEach((key, value) =>
        {
            testOutputHelper.WriteLine($"{key}: {value}");
        });

        var readonlyDict = new ReadOnlyDictionary<string, string>(dict);
        readonlyDict.ForEach(kv =>
        {
            testOutputHelper.WriteLine($"{kv.Key}: {kv.Value}");
        });
    }
}