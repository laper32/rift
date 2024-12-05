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
        //var packages = WorkspaceManager.GetAllPackages();
        //foreach (var package in packages)
        //{
        //    if (package.FindPlugin("Rift.Go") is not { } goPlugin)
        //    {
        //        continue;
        //    }
        //    // https://stackoverflow.com/a/73245455
        //    Console.WriteLine($"Inspecting {package.Name}");
        //    package.ForEachDependencies((_, reference) =>
        //    {
        //        Console.WriteLine($"{reference.Name}, version: {reference.Version}");
        //    });
        //}

        //Console.WriteLine("Now we generate go projects");
    }

    internal string GenerateGoModString()
    {

        return """
               
               module <Workspace.Name>
               
               go <Go.Version>
               
               require (
               	<DirectRequires>
               )
               """;
    }

    internal string GenerateGoWorkString()
    {
        return """
               go <Go.Version>
               
               use (
               	<LocalPackages>
               )
               
               """;
    }
}