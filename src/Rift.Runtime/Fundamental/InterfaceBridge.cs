using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Plugin;
using Rift.Runtime.Scripting;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Fundamental;

internal class InterfaceBridge(IServiceProvider provider)
{
    public IServiceProvider          Provider         => provider;
    public IShareSystemInternal      ShareSystem      => provider.GetRequiredService<IShareSystemInternal>();
    public IRuntimeInternal          Runtime          => provider.GetRequiredService<IRuntimeInternal>();
    public IWorkspaceManagerInternal WorkspaceManager => provider.GetRequiredService<IWorkspaceManagerInternal>();
    public IPluginManagerInternal    PluginManager    => provider.GetRequiredService<IPluginManagerInternal>();
    public IScriptManagerInternal    ScriptManager    => provider.GetRequiredService<IScriptManagerInternal>();
}