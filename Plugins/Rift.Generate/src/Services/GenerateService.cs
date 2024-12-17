using Rift.Runtime.Interfaces;

namespace Rift.Generate.Services;

public interface IGenerateListener
{
    void BeforeCall()
    {

    }

    void OnCall()
    {

    }

    void AfterCall()
    {

    }
}

public interface IGenerateService : IInterface
{
    event Action? Generate;

    void Call();

    void AddListener(IGenerateListener listener);
}

internal sealed class GenerateService : IGenerateService
{
    private static GenerateService _instance = null!;

    private readonly List<IGenerateListener> _listeners = [];

    public void AddListener(IGenerateListener listener)
    {

    }

    public GenerateService()
    {
        _instance = this;
    }

    public uint          InterfaceVersion => 1;
    public event Action? Generate;

    public void Call()
    {
        Console.WriteLine("Invocation success");
    }


    internal static void Invoke()
    {
        _instance.InvokeInternal();
    }

    private void InvokeInternal()
    {
        _listeners.ForEach(x =>
        {
            x.BeforeCall();

            x.OnCall();

            x.AfterCall();
        });

        Generate?.Invoke();
    }
}