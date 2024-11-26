namespace Rift.Runtime.API.Tasks;

/// <summary>
/// 表示命令行参数
/// </summary>
public interface ITaskArguments
{
    bool HasArgument(string name);

    ICollection<string> GetArguments(string name);

    IDictionary<string, ICollection<string>> GetArguments();
}