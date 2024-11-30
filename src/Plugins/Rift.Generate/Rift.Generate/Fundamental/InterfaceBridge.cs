using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Plugin;
using Rift.Runtime.Abstractions.Scripting;
using Rift.Runtime.Abstractions.Workspace;

namespace Rift.Generate.Fundamental;

internal class InterfaceBridge(Generate instance, IServiceProvider provider) : IPluginInterfaceBridge<Generate>
{
    public Generate          Instance         => instance;
    public IServiceProvider  Provider         => provider;
    public IRuntime          Runtime          => instance.Runtime;
    public IShareSystem      ShareSystem      => instance.ShareSystem;
    public IPluginManager    PluginManager    => instance.PluginManager;
    public IScriptManager    ScriptManager    => instance.ScriptManager;
    public IWorkspaceManager WorkspaceManager => instance.WorkspaceManager;
}