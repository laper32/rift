using System.CommandLine;

namespace Rift.Runtime.Tasks.Data;

public interface ITaskArgument
{
    public string Name { get; }
    public string Description { get; }
    internal Argument Value { get; }
}

internal class TaskArgument<T>(string name) : ITaskArgument
{
    private object? _defaultValue;
    private Argument<T>? _value;
    public string Name { get; init; } = name;

    public string Description { get; internal set; } = string.Empty;

    public Argument Value { get; private set; } = null!;

    internal void SetDefaultValue(object? defaultValue)
    {
        _defaultValue = defaultValue;
    }

    internal void Create()
    {
        if (_value is not null)
        {
            return;
        }

        _value = new Argument<T>(Name, Description);
        if (_defaultValue is not null)
        {
            _value.SetDefaultValue(_defaultValue);
        }

        Value = _value;
    }
}

public class TaskArgumentConfiguration<T>(string name)
{
    private readonly TaskArgument<T> _argument = new(name);

    public TaskArgumentConfiguration<T> Description(string description)
    {
        _argument.Description = description;
        return this;
    }

    public TaskArgumentConfiguration<T> Default(object? defaultValue)
    {
        _argument.SetDefaultValue(defaultValue);
        return this;
    }

    internal ITaskArgument Build()
    {
        _argument.Create();
        return _argument;
    }
}