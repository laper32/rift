using Microsoft.Extensions.DependencyInjection;
using Rift.Runtime.Commands;
using Rift.Runtime.Fundamental.Sharing;
using Rift.Runtime.Plugin;
using Rift.Runtime.Scripting;
using Rift.Runtime.Tasks;
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
    public ITaskManagerInternal      TaskManager      => provider.GetRequiredService<ITaskManagerInternal>();
    public ICommandManagerInternal   CommandManager   => provider.GetRequiredService<ICommandManagerInternal>();
}