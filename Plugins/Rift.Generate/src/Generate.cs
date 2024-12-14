using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Runtime.Interfaces;
using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;
using Rift.Runtime.Tasks;

[assembly: PluginShared]

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Generate initialized");
        var services = new ServiceCollection();
        services.AddSingleton<GenerateService>();

        var provider = services.BuildServiceProvider();
        InterfaceManager.AddInterface(provider.GetRequiredService<GenerateService>(), this);

        TaskManager.RegisterTask("rift.generate", config =>
        {
            config
                .AddOption<int>("hello-world", builder => builder
                    //.Long("hello-world")
                    .Description("114514")
                    .Short('w')
                    .Build())
                .AddArgument<int>("Arg1", builder => builder
                    .Description("1919810")
                    .Build())
                .SetIsCommand(true)
                .SetDeferException(true)
                .SetErrorHandler((exception, context) =>
                {
                    Console.WriteLine($"ErrorHandler, {exception.GetType()}, message: {exception.Message}");

                    return Task.CompletedTask;
                })
                .AddAction(() =>
                {
                    Console.WriteLine("12121");
                    GenerateService.Invoke();
                });
        });
        ScriptManager.AddLibrary("Rift.Generate");
        return base.OnLoad();
    }

    public override void OnUnload()
    {
        ScriptManager.RemoveLibrary("Rift.Generate");
        base.OnUnload();
    }
}

public static class Sth
{
    public static void Call()
    {
        Console.WriteLine("Call from Sth.Call()");
    }
}