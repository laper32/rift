using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Workspace;

namespace Rift.Runtime.API.Tasks;

public interface ITaskContext
{
    ITaskArguments    Arguments        { get; }
    IRuntime          Runtime          { get; }
    IWorkspaceManager WorkspaceManager { get; }
}