using Rift.Runtime.Interfaces;

namespace Rift.Generate.Services;

public interface IGenerateService : IInterface
{
    event Action? Generate;

    void Call();
}

internal sealed class GenerateService : IGenerateService
{
    public         uint            InterfaceVersion => 1;
    public event Action?           Generate;

    private static GenerateService _instance = null!;

    public GenerateService()
    {
        _instance = this;
    }

    public void Call()
    {
        Console.WriteLine("Invocation success");
    }


    internal static void Invoke() => _instance.InvokeInternal();

    private void InvokeInternal()
    {
        Generate?.Invoke();
    }
}