using Rift.Runtime.API.Manifest;

namespace Rift.Runtime.Workspace;
internal class VirtualPackage(IVirtualManifest virtualManifest, string manifestPath)
{
    public string Name         => virtualManifest.Name;
    public string ManifestPath => manifestPath;
    public string Root         => Directory.GetParent(ManifestPath)!.FullName;

    public string? Dependencies
    {
        get
        {
            if (virtualManifest.Dependencies is { } dependencies)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies);
            }

            return null;
        }
    }

    public string? Plugins
    {
        get
        {
            if (virtualManifest.Plugins is { } plugins)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, plugins);
            }

            return null;
        }
    }

    public string? Metadata
    {
        get
        {
            if (virtualManifest.Metadata is { } metadata)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, metadata);
            }

            return null;
        }
    }
}
