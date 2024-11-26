using Rift.Runtime.API.Tasks;

namespace Rift.Runtime.Tasks;

internal class TaskConfiguration : ITaskConfiguration
{
    public ITaskConfiguration SetDeferException(bool value)
    {
        Console.WriteLine($"DeferException => {value}");
        return this;
    }
}