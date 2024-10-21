using Microsoft.Extensions.Logging;

namespace Rift.Runtime.API.Fundamental;

public interface IRuntime
{
    ILoggerFactory Logger { get; }

    public static IRuntime Instance { get; protected set; }

    string ExecutablePath { get; }
    string InstallationPath { get; }
    string UserPath { get; }
}