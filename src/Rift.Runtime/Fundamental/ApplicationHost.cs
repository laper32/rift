// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Text;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Rift.Runtime.Fundamental;

/// <summary>
///     Represents the host for the application, providing useful variables and services.
/// </summary>
public sealed class ApplicationHost
{
    private const    int            MaxPath = 260;
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

    // https://learn.microsoft.com/en-us/windows/desktop/api/shlwapi/nf-shlwapi-pathfindonpathw
    // https://www.pinvoke.net/default.aspx/shlwapi.PathFindOnPath
    [DllImport("shlwapi.dll", CharSet = CharSet.Unicode, SetLastError = false)]
    private static extern bool PathFindOnPath([In] [Out] StringBuilder pszFile, [In] string[]? ppszOtherDirs);

    /// <summary>
    ///     Gets the full path of the given executable filename as if the user had entered this
    ///     executable in a shell. So, for example, the Windows PATH environment variable will
    ///     be examined. If the filename can't be found by Windows, null is returned. <br />
    ///     see: https://stackoverflow.com/questions/3855956/check-if-an-executable-exists-in-the-windows-path
    /// </summary>
    /// <param name="exeName"> The name of the executable. </param>
    /// <returns> The full path if successful, or null otherwise. </returns>
    /// <exception cref="ArgumentException"> Thrown when the executable name is too long. </exception>
    [SupportedOSPlatform("windows")]
    public static string? GetPathFromPathVariable(string exeName)
    {
        if (exeName.Length >= MaxPath)
        {
            throw new ArgumentException(
                $"The executable name '{nameof(exeName)}' must have less than {MaxPath} characters."
            );
        }

        var builder = new StringBuilder(exeName, MaxPath);
        return PathFindOnPath(builder, null) ? builder.ToString() : null;
    }
}