using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Workspace;

namespace Rift.Runtime.API.Scripting;

public interface IScriptContext
{
    IRuntime          Runtime          { get; }
    IWorkspaceManager WorkspaceManager { get; }
}