using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

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