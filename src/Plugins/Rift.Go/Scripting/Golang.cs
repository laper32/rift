using Rift.Go.Workspace;

namespace Rift.Go.Scripting;

public static class Golang
{
    public static void Configure(Action<GolangConfiguration> configure)
    {
        var configuration = new GolangConfiguration();
        configure(configuration);

        GolangWorkspaceService.Instance.SetGolangConfigure(configuration);
    }
}

/*

Golang
    .Version("1.22.3")

 */