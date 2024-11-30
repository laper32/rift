using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Workspace;

namespace Rift.Runtime.Abstractions.Tasks;

public interface ITaskContext
{
    ITaskArguments    Arguments        { get; }
    IRuntime          Runtime          { get; }
    IWorkspaceManager WorkspaceManager { get; }
}