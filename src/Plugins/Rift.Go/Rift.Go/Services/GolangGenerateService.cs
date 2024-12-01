using Rift.Go.Fundamental;

namespace Rift.Go.Services;


internal class GolangGenerateService
{
    internal static  GolangGenerateService Instance { get; private set; } = null!;
    private readonly InterfaceBridge       _bridge;

    public GolangGenerateService(InterfaceBridge bridge)
    {
        _bridge  = bridge;
        Instance = this;
    }

    public void PerformGolangGenerate()
    {
        var packages = _bridge.WorkspaceManager.GetAllPackages().Where(x => x.HasPlugin("Rift.Go"));
        
    }
}