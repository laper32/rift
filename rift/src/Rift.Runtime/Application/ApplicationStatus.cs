namespace Rift.Runtime.Application;

internal enum ApplicationStatus
{
    Unknown,
    KernelInit,
    KernelReady,
    RuntimeInit,
    RuntimeReady
}