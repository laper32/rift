using Rift.Generate.Abstractions;
using Rift.Generate.Fundamental;
using Rift.Runtime.Abstractions.Fundamental;

namespace Rift.Generate.Services;


internal class GenerateService : IGenerateService, ISharable
{
    public GenerateService(InterfaceBridge bridge)
    {
        bridge.ShareSystem.AddInterface(this, bridge.Instance);

        Instance = this;
    }
    public event Action? Generate;
    internal static GenerateService Instance { get; private set; } = null!;

    public void Invoke()
    {
        Console.WriteLine("Invokes generate registered events");
        Generate?.Invoke();
    }

    public string InterfaceName    => "IGenerateService";
    public uint   InterfaceVersion => 1;
}