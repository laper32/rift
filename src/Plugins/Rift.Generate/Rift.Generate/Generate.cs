using Rift.Runtime.Abstractions.Plugin;

namespace Rift.Generate;

// ReSharper disable once UnusedMember.Global
public class Generate : RiftPlugin
{
    public override bool OnLoad()
    {
        //if (TaskManager.Instance.FindTask("generate") is not { } command)
        //{
        //    Console.WriteLine("Failed to find `generate` command.");
        //    return false;
        //}

        //command.RegisterAction(() =>
        //{
        //    Console.WriteLine("aksdjjaklshdjklahsdjklhasjkdghajksdgjkladsg");
        //});

        //command.Action?.Invoke();

        var instances = WorkspaceManager.GetAllPackages().ToArray();
        Console.WriteLine($"Packages count: {instances.Length}");
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

        Console.WriteLine("Rift.Generate.OnLoad OK");
        Console.WriteLine($"Workspace root: {WorkspaceManager.Root}");
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