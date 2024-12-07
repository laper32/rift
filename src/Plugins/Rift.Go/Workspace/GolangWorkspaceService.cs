using System.Text.Json;
using Rift.Runtime.Workspace;

namespace Rift.Go.Workspace;

internal class GolangWorkspaceService : IWorkspaceListener
{
    private static   GolangWorkspaceService _instance = null!;
    private readonly List<GolangPackage>    _packages = [];

    internal static List<GolangPackage> Packages => _instance._packages;

    public GolangWorkspaceService(Plugin instance)
    {
        CollectGolangPackages();
        WorkspaceManager.AddListener(instance, this);
        _instance = this;
    }

    public void OnAllPackagesLoaded()
    {
        Console.WriteLine($"{Environment.GetEnvironmentVariable("GOPROXY")}");
    }

    internal static void DumpGolangPackages()
    {
        Console.WriteLine(
            JsonSerializer.Serialize(_instance._packages,
                new JsonSerializerOptions
                {
                    WriteIndented = true
                }
            )
        );
    }

    private void CollectGolangPackages()
    {
        foreach (var package in WorkspaceManager.GetAllPackages())
        {
            if (!package.HasPlugin("Rift.Go"))
            {
                continue;
            }

            _packages.Add(new GolangPackage(package));
        }
    }
}