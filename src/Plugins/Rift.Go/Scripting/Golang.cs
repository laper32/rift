using Rift.Go.Workspace;

namespace Rift.Go.Scripting;

public static class Golang
{
    public static void Configure(Action<GolangConfiguration> configure)
    {
        Plugin.HasSpecifiedServices();
    }
}
