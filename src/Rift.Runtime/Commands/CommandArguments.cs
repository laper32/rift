using Rift.Runtime.Collections.Generic;

namespace Rift.Runtime.Commands;

public class CommandArguments
{
    private readonly Dictionary<string, object?> _arguments = [];
    private readonly Dictionary<string, object?> _options   = [];

    internal void AddArguments(Dictionary<string, object?> arguments)
    {
        arguments.ForEach(_arguments.Add);
    }

    internal void AddOptions(Dictionary<string, object?> options)
    {
        options.ForEach(_options.Add);
    }

    public IReadOnlyDictionary<string, object?> GetArguments()
    {
        return _arguments;
    }

    public IReadOnlyDictionary<string, object?> GetOptions()
    {
        return _options;
    }
}