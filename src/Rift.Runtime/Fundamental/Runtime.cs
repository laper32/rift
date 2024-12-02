// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Rift.Runtime.Fundamental;

public sealed class Runtime
{
    public Runtime(IServiceProvider provider)
    {
        Logger           = provider.GetRequiredService<ILoggerFactory>();
        ExecutablePath   = Process.GetCurrentProcess().MainModule!.FileName;
        InstallationPath = Directory.GetParent(Directory.GetParent(ExecutablePath)!.FullName)!.FullName;
        UserPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
            Definitions.DirectoryIdentifier
        );
        Instance = this;
    }

    internal static Runtime        Instance         { get; private set; } = null!;
    public          ILoggerFactory Logger           { get; }
    public          string         ExecutablePath   { get; }
    public          string         InstallationPath { get; }
    public          string         UserPath         { get; }
}