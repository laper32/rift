namespace Rift.Go.Services;

internal class GolangGenerateService
{
    public GolangGenerateService()
    {
        Instance = this;
    }

    internal static GolangGenerateService Instance { get; private set; } = null!;

    public void PerformGolangGenerate()
    {
    }
}