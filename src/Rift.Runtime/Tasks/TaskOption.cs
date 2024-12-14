using System.CommandLine;

namespace Rift.Runtime.Tasks;

public interface ITaskOption
{
    /// <summary>
    /// 名字
    /// </summary>
    public string Name        { get; }

    /// <summary>
    /// 描述
    /// </summary>
    public string? Description { get; }

    /// <summary>
    /// 全名，也就是'--'的部分
    /// </summary>
    public string Long        { get; }

    /// <summary>
    /// 短名，也就是'-'的部分
    /// </summary>
    public char?  Short       { get; }

    internal Option Value { get; }
}

internal class TaskOption<T>(string name) : ITaskOption
{
    public string  Name        { get; init; } = name;
    public string? Description { get; internal set; }
    public string  Long        { get; internal set; } = name;
    public char?   Short       { get; internal set; }
    public Option  Value       { get; private set; } = null!;

    private Option<T>? _value;
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
        var longName  = $"--{Long}";
        var shortName = $"-{Short}";
        _value = new Option<T>([longName, shortName], description: Description);
        if (_defaultValue is not null)
        {
            _value.SetDefaultValue(_defaultValue);
        }
        Value = _value;
    }
}

public class TaskOptionBuilder<T>
{
    private TaskOption<T>? _option;

    private void CheckCreated()
    {
        if (_option is null)
        {
            throw new InvalidOperationException($"You must define option's `{nameof(Name)}` first!");
        }
    }

    public TaskOptionBuilder<T> Name(string name)
    {
        _option ??= new TaskOption<T>(name);
        return this;
    }

    public TaskOptionBuilder<T> Long(string longName)
    {
        CheckCreated();
        _option!.Long = longName;
        return this;
    }

    public TaskOptionBuilder<T> Short(char shortName)
    {
        CheckCreated();
        _option!.Short = shortName;
        return this;
    }

    public TaskOptionBuilder<T> Description(string description)
    {
        CheckCreated();
        _option!.Description = description;
        return this;
    }

    public TaskOptionBuilder<T> Default(object? defaultValue)
    {
        CheckCreated();
        _option!.SetDefaultValue(defaultValue);
        return this;
    }

    public ITaskOption Build()
    {
        CheckCreated();
        _option!.Create();
        return _option;
    }
}
