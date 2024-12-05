using System.Text.Json;
using Rift.Runtime.Workspace;

namespace Rift.Go.Workspace;

internal class GolangWorkspaceService
{
    private readonly List<GolangPackage> _packages = [];

    private GolangConfiguration? _config;

    internal static GolangWorkspaceService Instance { get; private set; } = null!;

    public GolangWorkspaceService()
    {
        CollectGolangPackages();
        Instance = this;
    }

    internal static void OnAddingReference(IPackageInstance package, PackageReference reference)
    {
        Console.WriteLine($"Ref: {package.Name}, refName: {reference.Name}");
    }

    internal void SetGolangConfigure(GolangConfiguration config)
    {
        if (_config is not null)
        {
            return;
        }

        _config = config;
    }

    internal void DumpGolangPackages()
    {
        Console.WriteLine(JsonSerializer.Serialize(_packages, new JsonSerializerOptions
        {
            WriteIndented = true
        }));
    }

    private void CollectGolangPackages()
    {
        foreach (var package in WorkspaceManager.GetAllPackages())
        {

            if (!package.HasPlugin("Rift.Go"))
            {
                continue;
            }

            Console.WriteLine($"Package: {package.Name}, HasPackage: {package.HasPlugin("Rift.Go")}");
            _packages.Add(new GolangPackage(package));

        }
        Console.WriteLine($"Collected packages count: {_packages.Count}");
        DumpGolangPackages();
    }
}