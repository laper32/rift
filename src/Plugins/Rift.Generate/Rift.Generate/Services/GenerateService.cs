using Rift.Generate.Abstractions;
using Rift.Generate.Fundamental;

namespace Rift.Generate.Services;


internal class GenerateService : IGenerateService
{

    public GenerateService(InterfaceBridge bridge)
    {
        Instance = this;
    }
    public event Action? Generate;
    internal static GenerateService Instance { get; private set; } = null!;

    public void Invoke()
    {
        Generate?.Invoke();
    }
}