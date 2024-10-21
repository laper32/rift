namespace Rift.Runtime.API.System;

public interface IScriptSystem
{
    public static IScriptSystem Instance { get; protected set; } = null!;
}