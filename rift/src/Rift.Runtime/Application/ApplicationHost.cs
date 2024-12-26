// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.Constants;

namespace Rift.Runtime.Application;

/// <summary>
///     Represents the host for the application, providing useful variables and services.
/// </summary>
public sealed partial class ApplicationHost
{
    private static ApplicationHost _instance = null!;

    private readonly InstallationInformation _installationInfo;
    private readonly ILoggerFactory          _logger;
    private readonly UserInformation         _userInfo;
    private          ApplicationStatus       _status;

    /// <summary>
    ///     Initializes a new instance of the <see cref="ApplicationHost" /> class.
    /// </summary>
    /// <param name="provider"> The service provider to get required services. </param>
    public ApplicationHost(IServiceProvider provider)
    {
        var executablePath   = Process.GetCurrentProcess().MainModule!.FileName;
        var installationPath = Directory.GetParent(Directory.GetParent(executablePath)!.FullName)!.FullName;
        var userPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
            Definitions.DirectoryIdentifier
        );

        _logger = provider.GetRequiredService<ILoggerFactory>();

        _installationInfo = new InstallationInformation(installationPath, executablePath);
        _userInfo         = new UserInformation(userPath);
        _status           = ApplicationStatus.Unknown;

        _instance = this;
    }

    /// <summary>
    ///     Gets the logger factory.
    /// </summary>
    public static ILoggerFactory Logger => _instance._logger;

    public static InstallationInformation InstallationInformation => _instance._installationInfo;

    public static UserInformation UserInformation => _instance._userInfo;

    public static ApplicationStatus Status
    {
        get => _instance._status;
        set => _instance._status = value;
    }

    /// <summary>
    ///     获取某一个可执行文件在PATH环境变量中的路径。 <br />
    ///     <remarks>
    ///         使用该函数时需要注意如下问题：<br />
    ///         1. 只负责返回路径，不负责判断该文件是否为可执行文件，这点请你一定注意。 <br />
    ///         2. LinWinMac对于界定什么是可执行文件的标准不一样，目前暂时不会做任何额外处理 <br />
    ///         （比如说判断如果是win默认加一个.exe后缀，lin不做处理一样，但实际上Lin/Mac很多是通过shell脚本来wrap可执行文件的）
    ///     </remarks>
    /// </summary>
    /// <param name="exeName"> 可执行文件的路径 </param>
    /// <returns> 目标可执行文件路径，为空则说明该文件不存在。 </returns>
    public static string? GetPathFromPathVariable(string exeName)
    {
        return OperatingSystem.IsWindows()
            ? GetPathFromPathVariableWindows(exeName)
            : GetPathFromPathVariableUnix(exeName);
    }
}