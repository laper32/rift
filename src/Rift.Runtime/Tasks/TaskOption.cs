using System.CommandLine;

namespace Rift.Runtime.Tasks;

public abstract class TaskOption(string name)
{
    /// <summary>
    /// 名字
    /// </summary>
    public string Name        { get; }      = name;

    /// <summary>
    /// 描述
    /// </summary>
    public string Description { get; set; } = string.Empty;
    /// <summary>
    /// 全名，也就是'--'的部分
    /// </summary>
    public string Long        { get; set; } = name;

    /// <summary>
    /// 短名，也就是'-'的部分
    /// </summary>
    public char?  Short       { get; set; }

}

public class TaskOption<T>(string name) : TaskOption(name)
{
    internal Option<T>? Value { get; private set; }
    private  object?    _defaultValue;

    internal void SetDefaultValue(object? defaultValue)
    {
        _defaultValue = defaultValue;
    }

    internal void Create()
    {
        if (Value is null)
        {
            return;
        }

        var      longName  = $"--{Long}";
        var      shortName = $"-{Short}";
        var commands  = new[] { longName, shortName };
        var opt = new Option<T>(aliases: commands, description: Description);
        opt.SetDefaultValue(_defaultValue);
        Value = opt;
    }
}

public class TaskOptionBuilder<T>(string name)
{
    private readonly TaskOption<T> _option = new(name);

    public TaskOptionBuilder<T> Long(string longName)
    {
        _option.Long = longName;
        return this;
    }

    public TaskOptionBuilder<T> Short(char shortName)
    {
        _option.Short = shortName;
        return this;
    }

    public TaskOptionBuilder<T> Description(string description)
    {
        _option.Description = description;
        return this;
    }

    public TaskOptionBuilder<T> Default(object? defaultValue)
    {
        _option.SetDefaultValue(defaultValue);
        return this;
    }

    public TaskOption<T> Build()
    {
        _option.Create();
        return _option;
    }
}
