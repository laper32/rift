using Rift.Runtime.Manifest;

namespace Rift.Runtime.Workspace;

public static class PackageInstanceExtensions
{
    public static bool HasPlugin(this IPackageInstance self, string pluginName)
    {
        return self.Plugins.ContainsKey(pluginName);
    }

    public static bool IsWorkspace(this IPackageInstance self)
    {
        var inner = ((PackageInstance)self).Value;
        if (inner.Type is not EMaybePackage.Virtual)
        {
            return false;
        }

        var pkgInner   = (MaybePackage<VirtualPackage>)inner;
        var virtualPkg = pkgInner.Value;
        var manifest   = virtualPkg.Value;
        return manifest.Type is EVirtualManifest.Workspace;
    }

    public static bool IsFolder(this IPackageInstance self)
    {
        var inner = ((PackageInstance)self).Value;
        if (inner.Type is not EMaybePackage.Virtual)
        {
            return false;
        }

        var pkgInner   = (MaybePackage<VirtualPackage>)inner;
        var virtualPkg = pkgInner.Value;
        var manifest   = virtualPkg.Value;
        return manifest.Type is EVirtualManifest.Folder;
    }

    public static bool IsProject(this IPackageInstance self)
    {
        var inner = ((PackageInstance)self).Value;
        if (inner.Type is not EMaybePackage.Package)
        {
            return false;
        }
        var pkgInner = (MaybePackage<Package>)inner;
        var pkg      = pkgInner.Value;
        var manifest = pkg.Value;
        return manifest.Type is EManifest.Project;
    }

    public static bool IsTarget(this IPackageInstance self)
    {
        var inner = ((PackageInstance)self).Value;
        if (inner.Type is not EMaybePackage.Package)
        {
            return false;
        }
        var pkgInner = (MaybePackage<Package>)inner;
        var pkg      = pkgInner.Value;
        var manifest = pkg.Value;
        return manifest.Type is EManifest.Target;
    }
}