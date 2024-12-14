using Rift.Runtime.Commands;

namespace Rift.Runtime.Tasks;

public interface ITaskData
{
    public TData GetOption<TData>(string   name) where TData : class;
    public TData GetArgument<TData>(string name) where TData : class;
}

internal class TaskData(CommandArguments args)
{
    public static TaskData FromCommandArguments(CommandArguments args)
    {
        return new TaskData(args);
    }

    private readonly IReadOnlyDictionary<string, object?> _options = args.GetOptions();
    private readonly IReadOnlyDictionary<string, object?> _arguments = args.GetArguments();

    public TData GetOption<TData>(string name) where TData : class
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

    public TData GetArgument<TData>(string name) where TData : class
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