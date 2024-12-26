using System.CommandLine;

namespace Rift.Runtime.Tasks.Data;

public interface ITaskOption
{
    /// <summary>
    ///     名字
    /// </summary>
    public string Name { get; }

    /// <summary>
    ///     描述
    /// </summary>
    public string? Description { get; }

    /// <summary>
    ///     全名，也就是'--'的部分
    /// </summary>
    public string Long { get; }

    /// <summary>
    ///     短名，也就是'-'的部分
    /// </summary>
    public char? Short { get; }

    internal Option Value { get; }
}

internal class TaskOption<T>(string name) : ITaskOption
{
    private object? _defaultValue;

    private Option<T>? _value;
    public string Name { get; init; } = name;
    public string? Description { get; internal set; }
    public string Long { get; internal set; } = name;
    public char? Short { get; internal set; }
    public Option Value { get; private set; } = null!;

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

        var longName = $"--{Long}";
        var shortName = $"-{Short}";
        _value = new Option<T>([longName, shortName], Description);
        if (_defaultValue is not null)
        {
            _value.SetDefaultValue(_defaultValue);
        }

        Value = _value;
    }
}

public class TaskOptionConfiguration<T>(string name)
{
    private readonly TaskOption<T> _option = new(name);

    public TaskOptionConfiguration<T> Long(string longName)
    {
        _option.Long = longName;
        return this;
    }

    public TaskOptionConfiguration<T> Short(char shortName)
    {
        _option.Short = shortName;
        return this;
    }

    public TaskOptionConfiguration<T> Description(string description)
    {
        _option.Description = description;
        return this;
    }

    public TaskOptionConfiguration<T> Default(object? defaultValue)
    {
        _option.SetDefaultValue(defaultValue);
        return this;
    }

    internal ITaskOption Build()
    {
        _option.Create();
        return _option;
    }
}