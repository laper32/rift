namespace Rift.Runtime.API.Tasks;

public interface IRiftDependentTask
{
    string Name       { get; }
    bool   IsRequired { get; }
}