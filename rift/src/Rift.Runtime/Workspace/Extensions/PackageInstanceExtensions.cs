using Rift.Runtime.Manifest.Real;
using Rift.Runtime.Manifest.Rift;
using Rift.Runtime.Manifest.Virtual;
using Rift.Runtime.Workspace.Fundamental;

namespace Rift.Runtime.Workspace.Extensions;

public static class PackageInstanceExtensions
{
    /// <summary>
    ///     判断该包是否有某个特定的插件
    /// </summary>
    /// <param name="self"> ~ </param>
    /// <param name="pluginName"> 插件名，需要注意大小写的问题 </param>
    /// <returns> ~ </returns>
    public static bool HasPlugin(this IPackageInstance self, string pluginName)
    {
        return self.Plugins.ContainsKey(pluginName);
    }

    /// <summary>
    ///     判断该包是否有某个特定的依赖
    /// </summary>
    /// <param name="self"> ~ </param>
    /// <param name="referenceName"> 依赖名，需要注意大小写的问题 </param>
    /// <returns> ~ </returns>
    public static bool HasDependency(this IPackageInstance self, string referenceName)
    {
        return self.Dependencies.ContainsKey(referenceName);
    }

    /// <summary>
    ///     判断该包是否为<see cref="VirtualPackage" />
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsVirtualPackage(this IPackageInstance self)
    {
        return ((PackageInstance) self).Value.Type is EMaybePackage.Virtual;
    }

    /// <summary>
    ///     判断该包是否为 <b> [workspace] </b>
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsWorkspace(this IPackageInstance self)
    {
        var inner = ((PackageInstance) self).Value;
        if (inner.Type is not EMaybePackage.Virtual)
        {
            return false;
        }

        var pkgInner   = (MaybePackage<VirtualPackage>) inner;
        var virtualPkg = pkgInner.Value;
        var manifest   = virtualPkg.Value;
        return manifest.Type is EVirtualManifest.Workspace;
    }

    /// <summary>
    ///     判断该包是否为 <b> [folder] </b>
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsFolder(this IPackageInstance self)
    {
        var inner = ((PackageInstance) self).Value;
        if (inner.Type is not EMaybePackage.Virtual)
        {
            return false;
        }

        var pkgInner   = (MaybePackage<VirtualPackage>) inner;
        var virtualPkg = pkgInner.Value;
        var manifest   = virtualPkg.Value;
        return manifest.Type is EVirtualManifest.Folder;
    }

    /// <summary>
    ///     判断该包是否为 <see cref="Package" />
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsPackage(this IPackageInstance self)
    {
        return ((PackageInstance) self).Value.Type is EMaybePackage.Package;
    }

    /// <summary>
    ///     判断该包是否为 <b> [project] </b>
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsProject(this IPackageInstance self)
    {
        var inner = ((PackageInstance) self).Value;
        if (inner.Type is not EMaybePackage.Package)
        {
            return false;
        }

        var pkgInner = (MaybePackage<Package>) inner;
        var pkg      = pkgInner.Value;
        var manifest = pkg.Value;
        return manifest.Type is EManifest.Project;
    }

    /// <summary>
    ///     判断该包是否为 <b> [target] </b>
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsTarget(this IPackageInstance self)
    {
        var inner = ((PackageInstance) self).Value;
        if (inner.Type is not EMaybePackage.Package)
        {
            return false;
        }

        var pkgInner = (MaybePackage<Package>) inner;
        var pkg      = pkgInner.Value;
        var manifest = pkg.Value;
        return manifest.Type is EManifest.Target;
    }

    /// <summary>
    ///     判断该包是否为 <see cref="RiftPackage" />
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsRiftPackage(this IPackageInstance self)
    {
        return ((PackageInstance) self).Value.Type is EMaybePackage.Rift;
    }

    /// <summary>
    ///     判断该包是否为 <b> [plugin] </b>
    /// </summary>
    /// <param name="self"> </param>
    /// <returns> ~ </returns>
    public static bool IsPlugin(this IPackageInstance self)
    {
        var inner = ((PackageInstance) self).Value;
        if (inner.Type is not EMaybePackage.Package)
        {
            return false;
        }

        var pkgInner = (MaybePackage<RiftPackage>) inner;
        var pkg      = pkgInner.Value;
        var manifest = pkg.Value;
        return manifest.Type is ERiftManifest.Plugin;
    }
}