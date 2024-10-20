using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;

namespace Rift.Runtime.Fundamental;

internal interface IRuntimeInternal : IRuntime;

internal class Runtime(InterfaceBridge bridge) : IRuntimeInternal
{
    public ILoggerFactory Logger => bridge.Logger;
}