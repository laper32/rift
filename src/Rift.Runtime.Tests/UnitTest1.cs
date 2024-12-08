using System.Collections.ObjectModel;
using System.Data;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Text;
using Rift.Runtime.Collections.Generic;
using Rift.Runtime.Commands;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Tasks;
using Semver;
using Xunit.Abstractions;

namespace Rift.Runtime.Tests;

public class UnitTest1(ITestOutputHelper testOutputHelper) : RuntimeSetup
{
    [Fact]
    public void Test1()
    {
        TaskManager.RegisterTask("rift.new", config => { config.SetIsCommand(true); });
        TaskManager.RegisterTask("rift.new.classlib", config => { config.SetIsCommand(true); });
        var commands = TaskManager.GetMarkedAsCommandTasks();
        var entries = UserCommand.Build(commands);

        //CommandManager.Instance.ExecuteCommand(Environment.GetCommandLineArgs());
    }

    [Fact]
    public void DictionaryForEach()
    {
        var dict = new Dictionary<string, string>
        {
            { "a", "b" },
            { "c", "d" }
        };
        dict.ForEach((key, value) => { testOutputHelper.WriteLine($"{key}: {value}"); });

        var readonlyDict = new ReadOnlyDictionary<string, string>(dict);
        readonlyDict.ForEach(kv => { testOutputHelper.WriteLine($"{kv.Key}: {kv.Value}"); });
    }


    [Fact]
    public void SetPackageReferenceRefWorkspace()
    {
        var possibleGoVersions = new List<string>
        {
            "1.22.2", //GolangEnvironment.Version,
            "1.24.0", //package.Configuration.GetGolangVersion(),
            ""        //Environment.GetEnvironmentVariable("Go.Version") ?? ""
        }.Where(x => !string.IsNullOrEmpty(x)).ToList();

        var goVersions = new List<SemVersion>();
        possibleGoVersions.ForEach(x =>
        {
            goVersions.Add(SemVersion.Parse(x));
        });

        goVersions.Sort(SemVersion.SortOrderComparer);
        goVersions.Reverse();
        goVersions.ForEach(x =>
        {
            testOutputHelper.WriteLine($"{x}");
        });

        //possibleGoVersions.ForEach(x =>
        //{
        //    testOutputHelper.WriteLine($"{x} => IsNullOrEmpty: {string.IsNullOrEmpty(x)}");
        //});
    }
}