using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Tasks;
using Rift.Runtime.Abstractions.Workspace;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

internal class TaskContext(InterfaceBridge bridge) : ITaskContext
{
    public ITaskArguments    Arguments        { get; }
    public IRuntime          Runtime          => bridge.Runtime;
    public IWorkspaceManager WorkspaceManager => bridge.WorkspaceManager;
}