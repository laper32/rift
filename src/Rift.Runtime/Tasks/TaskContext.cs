
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Tasks;

internal class TaskContext(InterfaceBridge bridge) : ITaskContext
{
    public ITaskArguments    Arguments        { get; }
    public IRuntime          Runtime          => bridge.Runtime;
}

public interface ITaskContext
{
    ITaskArguments Arguments { get; }
}