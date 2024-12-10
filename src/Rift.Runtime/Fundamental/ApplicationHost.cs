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

    /// <summary>
    /// 获取某一个可执行文件在PATH环境变量中的路径。 <br/>
    /// <remarks>
    /// 使用该函数时需要注意如下问题：<br/>
    /// 1. 只负责返回路径，不负责判断该文件是否为可执行文件，这点请你一定注意。 <br/>
    /// 2. LinWinMac对于界定什么是可执行文件的标准不一样，目前暂时不会做任何额外处理 <br/>
    /// （比如说判断如果是win默认加一个.exe后缀，lin不做处理一样，但实际上Lin/Mac很多是通过shell脚本来wrap可执行文件的）
    /// </remarks>
    /// </summary>
    /// <param name="exeName">可执行文件的路径</param>
    /// <returns>目标可执行文件路径，为空则说明该文件不存在。</returns>
    public static string? GetPathFromPathVariable(string exeName)
    {
        return OperatingSystem.IsWindows()
            ? GetPathFromPathVariableWindows(exeName)
            : GetPathFromPathVariableUnix(exeName);
    }
}