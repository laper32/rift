using System.Text.Json;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Task;
using Rift.Runtime.API.Workspace;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
public class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        if (ITaskManager.Instance.FindTask("generate") is not { } command)
        {
            Console.WriteLine("Failed to find `generate` command.");
            return false;
        }
        command.RegisterAction(() =>
        {
            Console.WriteLine("aksdjjaklshdjklahsdjklhasjkdghajksdgjkladsg");
        });
        var instances = IWorkspaceManager.Instance.GetAllPackages();

        foreach (var instance in instances)
        {
            if (instance.GetExtensionField("build") is not { } field)
            {
                continue;
            }

            if (field.GetString() is not { } fieldStr)
            {
                continue;
            }
            Console.WriteLine($"name: {instance.Name} => {fieldStr}");
        }

        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        Console.WriteLine("Rift.Generate.OnAllLoaded Ok.");
        base.OnAllLoaded();
    }

    public override void OnUnload()
    {
        Console.WriteLine("Rift.Generate.OnUnload OK.");
        base.OnUnload();
    }
}