using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Scripting;
using Rift.Runtime.Abstractions.Workspace;

namespace Rift.Runtime.Abstractions.Plugin;

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
