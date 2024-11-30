using Rift.Runtime.Abstractions.Fundamental;

namespace Rift.Generate.Abstractions;

public interface IGenerateService : ISharable
{
    public event Action? Generate;

}