using Rift.Runtime.Commands.Data;

namespace Rift.Runtime.Tasks.Data;

public interface ITaskData
{
    public TData GetOption<TData>(string name);
    public TData GetArgument<TData>(string name);
}

internal class TaskData(CommandArguments args) : ITaskData
{
    private readonly IReadOnlyDictionary<string, object?> _arguments = args.GetArguments();
    private readonly IReadOnlyDictionary<string, object?> _options = args.GetOptions();

    public TData GetOption<TData>(string name)
    {
        if (!_options.TryGetValue(name, out var value))
        {
            throw new InvalidOperationException($"Option {name} not found");
        }

        if (value is TData typedValue)
        {
            return typedValue;
        }

        throw new InvalidOperationException($"{name}'s type is not {typeof(TData)}");
    }

    public TData GetArgument<TData>(string name)
    {
        if (!_arguments.TryGetValue(name, out var value))
        {
            throw new InvalidOperationException($"Argument {name} not found");
        }

        if (value is TData typedValue)
        {
            return typedValue;
        }

        throw new InvalidOperationException($"{name}'s type is not {typeof(TData)}");
    }
}