// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Rift.Runtime.Fundamental;

/// <summary>
///     Represents the host for the application, providing useful variables and services.
/// </summary>
public sealed partial class ApplicationHost
{
    private readonly string         _executablePath;
    private readonly string         _installationPath;
    private readonly ILoggerFactory _logger;
    private readonly string         _userPath;

    /// <summary>
    ///     Initializes a new instance of the <see cref="ApplicationHost" /> class.
    /// </summary>
    /// <param name="provider"> The service provider to get required services. </param>
    public ApplicationHost(IServiceProvider provider)
    {
        _logger           = provider.GetRequiredService<ILoggerFactory>();
        _executablePath   = Process.GetCurrentProcess().MainModule!.FileName;
        _installationPath = Directory.GetParent(Directory.GetParent(_executablePath)!.FullName)!.FullName;
        _userPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
            Definitions.DirectoryIdentifier
        );
        Instance = this;
    }

    /// <summary>
    ///     Gets the singleton instance of the <see cref="ApplicationHost" /> class.
    /// </summary>
    internal static ApplicationHost Instance { get; private set; } = null!;

    /// <summary>
    ///     Gets the logger factory.
    /// </summary>
    public static ILoggerFactory Logger => Instance._logger;

    /// <summary>
    ///     Gets the path of the executable.
    /// </summary>
    public static string ExecutablePath => Instance._executablePath;

    /// <summary>
    ///     Gets the installation path.
    /// </summary>
    public static string InstallationPath => Instance._installationPath;

    /// <summary>
    ///     Gets the user path.
    /// </summary>
    public static string UserPath => Instance._userPath;

    
}