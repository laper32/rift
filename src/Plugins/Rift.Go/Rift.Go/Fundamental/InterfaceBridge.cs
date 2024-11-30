using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Abstractions;
using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Plugin;
using Rift.Runtime.Abstractions.Scripting;
using Rift.Runtime.Abstractions.Workspace;

namespace Rift.Go.Fundamental;

internal class InterfaceBridge(Golang instance, IServiceProvider provider) : IPluginInterfaceBridge<Golang>
{
    public Golang            Instance         => instance;
    public IServiceProvider  Provider         => provider;
    public IRuntime          Runtime          => instance.Runtime;
    public IShareSystem      ShareSystem      => instance.ShareSystem;
    public IPluginManager    PluginManager    => instance.PluginManager;
    public IScriptManager    ScriptManager    => instance.ScriptManager;
    public IWorkspaceManager WorkspaceManager => instance.WorkspaceManager;
}