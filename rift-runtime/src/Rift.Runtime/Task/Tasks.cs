// ReSharper disable UnusedMember.Global

namespace Rift.Runtime.Task;

public static class Tasks
{
    public delegate void TaskAction();
    public static void Implement(string taskName, TaskAction action)
    {
        action();
    }
}