// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;

namespace Rift.Runtime.API.Fundamental;

public abstract class Runtime
{
    protected Runtime()
    {
        Instance = this;
    }

    public abstract ILoggerFactory Logger { get; }

    public static Runtime Instance { get; protected set; } = null!;


    public abstract string ExecutablePath { get; }
    public abstract string InstallationPath { get; }
    public abstract string UserPath { get; }
}