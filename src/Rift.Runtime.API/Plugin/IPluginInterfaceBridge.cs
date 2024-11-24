using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;

namespace Rift.Runtime.API.Plugin;

public interface IPluginInterfaceBridge<out T> where T : RiftPlugin
{
    T                 Instance         { get; }
    IServiceProvider  Provider         { get; }
    IRuntime          Runtime          { get; }
    IShareSystem      ShareSystem      { get; }
    IPluginManager    PluginManager    { get; }
    IScriptManager    ScriptManager    { get; }
    IWorkspaceManager WorkspaceManager { get; }
}
