using System.Text.Json;
using Rift.Runtime.Workspace;

namespace Rift.Go.Workspace;

internal class GolangWorkspaceService
{
    private readonly List<GolangPackage> _packages = [];

    private static GolangWorkspaceService _instance  = null!;

    public GolangWorkspaceService()
    {
        CollectGolangPackages();
        _instance = this;
    }

    internal static void OnAddingReference(IPackageInstance package, PackageReference reference) =>
        _instance.OnAddingReferenceInternal(package, reference);

    internal static void DumpGolangPackages()
        => _instance.DumpGolangPackagesInternal();

    private void DumpGolangPackagesInternal()
    {
        Console.WriteLine(JsonSerializer.Serialize(_packages, new JsonSerializerOptions
        {
            WriteIndented = true
        }));
    }


    private void OnAddingReferenceInternal(IPackageInstance self, PackageReference reference)
    {
        if (_packages.FirstOrDefault(x => x.Instance.Equals(self)) is not { } package)
        {
            return;
        }

        package.Dependencies.Add(reference.Name, reference);
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