// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Commands;

public abstract class CommandManager
{
    public static CommandManager Instance { get; protected set; } = null!;

    protected CommandManager()
    {
        Instance = this;
    }

    public abstract void CallOnce();
}
