// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental.Interop;
using Rift.Runtime.Fundamental.Interop.Natives;

namespace Rift.Runtime.Fundamental;

internal class RuntimeInternal : API.Fundamental.Runtime
{
    public RuntimeInternal(IServiceProvider provider)
    {
        Logger   = provider.GetRequiredService<ILoggerFactory>();
        Instance = this;
    }

    public new static RuntimeInternal Instance { get; private set; } = null!;

    public override ILoggerFactory Logger { get; }
    public override string ExecutablePath
    {
        get
        {
            unsafe
            {
                return NativeString.ReadFromPointer(Core.GetExecutablePath());
            }
        }
    }

    public override string InstallationPath => Directory.GetParent(Directory.GetParent(ExecutablePath)!.FullName)!.FullName;
    public override string UserPath => Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".rift");
}