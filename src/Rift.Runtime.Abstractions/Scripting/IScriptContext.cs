using Rift.Runtime.Abstractions.Fundamental;
using Rift.Runtime.Abstractions.Workspace;

namespace Rift.Runtime.Abstractions.Scripting;

public interface IScriptContext
{
    IRuntime          Runtime          { get; }
    IWorkspaceManager WorkspaceManager { get; }
}