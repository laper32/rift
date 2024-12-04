using Rift.Runtime.Collections.Generic;
using Rift.Runtime.Workspace;

namespace Rift.Go.Services;

internal class GolangGenerateService
{
    public GolangGenerateService()
    {
        Instance = this;
    }

    internal static GolangGenerateService Instance { get; private set; } = null!;

    public static void PerformGolangGenerate()
    {
        var packages = WorkspaceManager.GetAllPackages();
        foreach (var package in packages)
        {
            Console.WriteLine($"Inspecting {package.Name}");
            package.Dependencies.ForEach((_, reference) =>
            {
                Console.WriteLine($"{reference.Name}");
            });
            //package.Dependencies.ForEach((_, dep) =>
            //{
            //    var golangImport = dep as GolangImport;
            //    Console.WriteLine($"golangImport: {golangImport?.Name}");
            //});
        }

        Console.WriteLine("Now we generate go projects");
    }
}