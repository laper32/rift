namespace Rift.Runtime.Tasks;

public class TaskNotFoundException(string message = "") : Exception(message);