using System.CommandLine;

namespace Rift.Runtime.Tasks;

public interface ITaskArgument
{
    public   string   Name        { get; }
    public   string   Description { get; }
    internal Argument Value       { get; }
}

internal class TaskArgument<T>(string name) : ITaskArgument
{
    public string Name        { get; init; } = name;

    public string Description { get; internal set; } = string.Empty;

    public  Argument     Value { get; private set; } = null!;
    private Argument<T>? _value;
    

    private object? _defaultValue;

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

        _value = new Argument<T>(name: Name, description: Description);
        if (_defaultValue is not null)
        {
            _value.SetDefaultValue(_defaultValue);
        }

        Value = _value;
    }
}

public class TaskArgumentBuilder<T>
{
    private TaskArgument<T>? _argument;

    private void CheckCreated()
    {
        if (_argument is null)
        {
            throw new InvalidOperationException($"You must define argument's `{nameof(Name)}` first!");
        }
    }

    public TaskArgumentBuilder<T> Name(string name)
    {
        _argument ??= new TaskArgument<T>(name);
        return this;
    }

    public TaskArgumentBuilder<T> Description(string description)
    {
        CheckCreated();
        _argument!.Description = description;
        return this;
    }

    public TaskArgumentBuilder<T> Default(object? defaultValue)
    {
        CheckCreated();
        _argument!.SetDefaultValue(defaultValue);
        return this;
    }

    public ITaskArgument Build()
    {
        CheckCreated();
        _argument!.Create();
        return _argument;
    }
}

