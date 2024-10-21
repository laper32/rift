using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.System;

namespace Rift.Runtime.System;

internal interface IScriptSystemInternal : IScriptSystem, IInitializable;

internal class ScriptSystem : IScriptSystemInternal
{
    public ScriptSystem()
    {
        IScriptSystem.Instance = this;
    }

    private bool _init;
    private bool _shutdown;
    public bool Init()
    {
        _init = true;
        _shutdown = false;
        return true;
    }

    public void Shutdown()
    {
        _shutdown = true;
        _init = false;
    }
}