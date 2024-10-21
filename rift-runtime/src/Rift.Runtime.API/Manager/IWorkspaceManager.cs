namespace Rift.Runtime.API.Manager;

public interface IWorkspaceManager
{
    public static IWorkspaceManager Instance { get; protected set; }

    void ParseWorkspace();
}