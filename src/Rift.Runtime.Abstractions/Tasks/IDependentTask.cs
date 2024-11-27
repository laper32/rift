namespace Rift.Runtime.Abstractions.Tasks;

public interface IDependentTask
{
    string Name       { get; }
    bool   IsRequired { get; }
}