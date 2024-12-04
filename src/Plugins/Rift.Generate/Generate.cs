using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;
using Rift.Runtime.Tasks;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
internal class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        Console.WriteLine("Rift.Generate initialized");
        //var services = new ServiceCollection();
        //services.AddSingleton(this);
        //services.AddSingleton<InterfaceBridge>();
        //services.AddSingleton<IGenerateService, GenerateService>();

        //var provider = services.BuildServiceProvider();
        //provider.GetRequiredService<IGenerateService>();

        TaskManager.Instance.RegisterTask("rift.generate", config =>
        {
            config
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
                });
        });
        ScriptManager.AddNamespace("Rift.Generate");
        ScriptManager.AddLibrary("Rift.Generate");
        return base.OnLoad();
    }

    public override void OnUnload()
    {
        ScriptManager.RemoveLibrary("Rift.Generate");
        ScriptManager.RemoveNamespace("Rift.Generate");
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