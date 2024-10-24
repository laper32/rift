// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Fundamental.Interop;
using Rift.Runtime.Fundamental.Interop.Natives;

namespace Rift.Runtime.Fundamental;

internal interface IRuntimeInternal : IRuntime;

internal class Runtime : IRuntimeInternal
{
    public Runtime(IServiceProvider provider)
    {
        Logger = provider.GetRequiredService<ILoggerFactory>();
        IRuntime.Instance = this;
    }
    public ILoggerFactory Logger { get; }
    public string ExecutablePath
    {
        get
        {
            unsafe
            {
                return NativeString.ReadFromPointer(Core.GetExecutablePath());
            }
        }
    }

    public string InstallationPath => Directory.GetParent(Directory.GetParent(ExecutablePath)!.FullName)!.FullName;
    public string UserPath => Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".rift");
}