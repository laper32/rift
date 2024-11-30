namespace Rift.Runtime.Abstractions.Tasks;

public class TaskNotFoundException(string message = "") : Exception(message);
