namespace Rift.Runtime.Tasks;

/// <summary>
///     表示命令行参数
/// </summary>
public interface ITaskArguments
{
    bool HasArgument(string name);

    ICollection<string> GetArguments(string name);

    IDictionary<string, ICollection<string>> GetArguments();
}

internal class TaskArguments : ITaskArguments
{
    public bool HasArgument(string name)
    {
        throw new NotImplementedException();
    }

    public ICollection<string> GetArguments(string name)
    {
        throw new NotImplementedException();
    }

    public IDictionary<string, ICollection<string>> GetArguments()
    {
        throw new NotImplementedException();
    }
}