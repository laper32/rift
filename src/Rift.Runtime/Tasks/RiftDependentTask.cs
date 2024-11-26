using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

internal class RiftDependentTask : IRiftDependentTask
{
    public string Name       { get; }
    public bool   IsRequired { get; }

    public RiftDependentTask(string name, bool required)
    {
        ArgumentNullException.ThrowIfNull(name, nameof(name));
        Name       = name;
        IsRequired = required;
    }
}