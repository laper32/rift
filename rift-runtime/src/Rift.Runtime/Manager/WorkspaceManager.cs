using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manager;
using Rift.Runtime.API.Schema;
using Tomlyn;

namespace Rift.Runtime.Manager;

internal interface IWorkspaceManagerInternal : IWorkspaceManager, IInitializable;

public class WorkspaceManager : IWorkspaceManagerInternal
{
    public WorkspaceManager()
    {
        IWorkspaceManager.Instance = this;
    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {
    }

    public void ParseWorkspace()
    {
        var cwd = Environment.CurrentDirectory;
        var manifestPath = Path.Combine(cwd, Definitions.ManifestIdentifier);
        var text = File.ReadAllText(manifestPath);

        var folder = Toml.ToModel<TomlManifest>(text);
        Console.WriteLine(folder.Folder);
    }
}