// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;

namespace Rift.Runtime.API.Fundamental;

public interface IRuntime
{
    ILoggerFactory Logger           { get; }
    string         ExecutablePath   { get; }
    string         InstallationPath { get; }
    string         UserPath         { get; }
}