// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Rift.Runtime.Fundamental;

public sealed class ApplicationHost
{
    public ApplicationHost(IServiceProvider provider)
    {
        _logger = provider.GetRequiredService<ILoggerFactory>();
        _executablePath = Process.GetCurrentProcess().MainModule!.FileName;
        _installationPath = Directory.GetParent(Directory.GetParent(_executablePath)!.FullName)!.FullName;
        _userPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
            Definitions.DirectoryIdentifier
        );
        Instance = this;
    }
    private readonly string         _executablePath;
    private readonly string         _installationPath;
    private readonly string         _userPath;
    private readonly ILoggerFactory _logger;

    internal static ApplicationHost Instance         { get; private set; } = null!;
    public static   ILoggerFactory  Logger           => Instance._logger;
    public static   string          ExecutablePath   => Instance._executablePath;
    public static   string          InstallationPath => Instance._installationPath;
    public static   string          UserPath         => Instance._userPath;

}