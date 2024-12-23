namespace Rift.Runtime.Application;

public enum ApplicationStatus
{
    Unknown,
    KernelInit,
    KernelReady,
    RuntimeInit,
    RuntimeReady
}