
namespace Rift.Runtime.Tasks;

public interface IDependentTask
{
    string Name       { get; }
    bool   IsRequired { get; }
}

internal class DependentTask : IDependentTask
{
    public string Name       { get; }
    public bool   IsRequired { get; }

    public DependentTask(string name, bool required)
    {
        ArgumentNullException.ThrowIfNull(name, nameof(name));
        Name       = name;
        IsRequired = required;
    }
}