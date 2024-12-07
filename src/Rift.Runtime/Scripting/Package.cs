using Rift.Runtime.Workspace;

namespace Rift.Runtime.Scripting;

// ReSharper disable UnusedMember.Global

public static class Package
{
    public static void Configure(Action<PackageConfiguration> configure)
    {
        WorkspaceManager.ConfigurePackage(configure);
        //WorkspaceManager.AddPluginForPackage(plugin);

    }
}