using System.CommandLine;

namespace Rift.Runtime.Tasks;

public abstract class TaskArgument(string name)
{
    public string Name { get; } = name;
}

public class TaskArgument<T> : TaskArgument
{
    private readonly Argument<T> _argument;
    public TaskArgument(string name) : base(name)
    {
        var argument = new Argument<T>(name);
        _argument = argument;
    }

}