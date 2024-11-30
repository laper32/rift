using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

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