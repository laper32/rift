
namespace Rift.Generate.Services;

public interface IGenerateService
{

}

internal class GenerateService : IGenerateService
{
    public GenerateService()
    {
        //bridge.InterfaceManager.AddInterface(this, bridge.Instance);

        Instance = this;
    }
    public event Action? Generate;


    internal static GenerateService Instance { get; private set; } = null!;

    public void Invoke()
    {
        Console.WriteLine("Invokes generate registered events");
        Generate?.Invoke();
    }

    public string InterfaceName => "IGenerateService";
    public uint InterfaceVersion => 1;
}