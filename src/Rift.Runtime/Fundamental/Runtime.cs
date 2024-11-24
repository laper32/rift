// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;

namespace Rift.Runtime.Fundamental;

internal interface IRuntimeInternal : IRuntime;

internal class Runtime(InterfaceBridge bridge) : IRuntimeInternal
{
    public ILoggerFactory Logger         => bridge.Provider.GetRequiredService<ILoggerFactory>();
    public string         ExecutablePath => Process.GetCurrentProcess().MainModule!.FileName;
    public string InstallationPath => Directory.GetParent(Directory.GetParent(ExecutablePath)!.FullName)!.FullName;
    public string UserPath => Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".rift");
}