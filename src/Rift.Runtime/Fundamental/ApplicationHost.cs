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

public sealed class ApplicationHost
{
    private readonly string         _executablePath;
    private readonly string         _installationPath;
    private readonly ILoggerFactory _logger;
    private readonly string         _userPath;

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

    internal static ApplicationHost Instance         { get; private set; } = null!;
    public static   ILoggerFactory  Logger           => Instance._logger;
    public static   string          ExecutablePath   => Instance._executablePath;
    public static   string          InstallationPath => Instance._installationPath;
    public static   string          UserPath         => Instance._userPath;

    // https://learn.microsoft.com/en-us/windows/desktop/api/shlwapi/nf-shlwapi-pathfindonpathw
    // https://www.pinvoke.net/default.aspx/shlwapi.PathFindOnPath
    [DllImport("shlwapi.dll", CharSet = CharSet.Unicode, SetLastError = false)]
    static extern bool PathFindOnPath([In, Out] StringBuilder pszFile, [In] string[]? ppszOtherDirs);

    private const int MaxPath = 260;

    /// <summary>
    /// Gets the full path of the given executable filename as if the user had entered this
    /// executable in a shell. So, for example, the Windows PATH environment variable will
    /// be examined. If the filename can't be found by Windows, null is returned. <br/>
    /// 
    /// see: https://stackoverflow.com/questions/3855956/check-if-an-executable-exists-in-the-windows-path
    /// </summary>
    /// <param name="exeName"></param>
    /// <returns>The full path if successful, or null otherwise.</returns>
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