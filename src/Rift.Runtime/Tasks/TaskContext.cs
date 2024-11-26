using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Tasks;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

internal class TaskContext(InterfaceBridge bridge) : ITaskContext
{
    public ITaskArguments    Arguments        { get; }
    public IRuntime          Runtime          => bridge.Runtime;
    public IWorkspaceManager WorkspaceManager => bridge.WorkspaceManager;
}