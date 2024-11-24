using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Task;
using Rift.Runtime.API.Workspace;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
public class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        if (TaskManager.Instance.FindTask("generate") is not { } command)
        {
            Console.WriteLine("Failed to find `generate` command.");
            return false;
        }
        
        command.RegisterAction(() =>
        {
            Console.WriteLine("aksdjjaklshdjklahsdjklhasjkdghajksdgjkladsg");
        });

        command.Action?.Invoke();
        
        var instances = WorkspaceManager.Instance.GetAllPackages();

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

        Console.WriteLine($"Workspace root: {WorkspaceManager.Instance.Root}");

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