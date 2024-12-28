using Rift.Runtime.Workspace.Abstractions;
using Rift.Runtime.Workspace.Managers;
using System.Text.Json;
using Rift.Runtime.Workspace.Extensions;
using Rift.Go.Workspace.Fundamental;

namespace Rift.Go.Workspace.Managers;

internal class GolangWorkspaceManager : IWorkspaceListener
{
    private static GolangWorkspaceManager _instance = null!;
    private readonly List<GolangPackage> _packages = [];

    public GolangWorkspaceManager(GolangPlugin instance)
    {
        WorkspaceManager.AddListener(instance, this);
        _instance = this;
    }

    internal List<GolangPackage> Packages => _instance._packages;

    public void OnAllPackagesLoaded()
    {
        CollectGolangPackages();
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