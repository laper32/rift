namespace Rift.Runtime.Abstractions.Tasks;

public interface IRiftDependentTask
{
    string Name       { get; }
    bool   IsRequired { get; }
}