using Rift.Runtime.Commands;

namespace Rift.Runtime.Tasks;

public interface ITaskData
{
    public TData GetOption<TData>(string   name);
    public TData GetArgument<TData>(string name);
}

internal class TaskData(CommandArguments args) : ITaskData
{
    public static TaskData FromCommandArguments(CommandArguments args)
    {
        return new TaskData(args);
    }

    private readonly IReadOnlyDictionary<string, object?> _options = args.GetOptions();
    private readonly IReadOnlyDictionary<string, object?> _arguments = args.GetArguments();

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