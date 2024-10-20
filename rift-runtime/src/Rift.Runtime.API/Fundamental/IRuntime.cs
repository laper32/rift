using Microsoft.Extensions.Logging;

namespace Rift.Runtime.API.Fundamental;

public interface IRuntime
{
    ILoggerFactory Logger { get; }
}