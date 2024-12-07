namespace Rift.Runtime.Workspace;

public interface IWorkspaceListener
{
    void OnLoadPackage(IPackageInstance package);

    void OnAddingPackageReference(IPackageInstance package, PackageReference reference);
    void OnPackageReferenceAdded(IPackageInstance package, PackageReference reference);
    void OnAllPackageLoaded();
}