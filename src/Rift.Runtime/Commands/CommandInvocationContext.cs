using System.CommandLine.Invocation;

namespace Rift.Runtime.Commands;

internal class CommandInvocationContext(InvocationContext context)
{
    public InvocationContext Context { get; } = context;
}