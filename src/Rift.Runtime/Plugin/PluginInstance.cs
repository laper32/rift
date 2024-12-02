// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Reflection;
using Microsoft.Extensions.Logging;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Fundamental.Extensions;

namespace Rift.Runtime.Plugin;

internal class PluginInstance(InterfaceBridge bridge, PluginContext context)
{
    private readonly PluginIdentity _identity = context.Identity;
    private readonly Assembly?      _entry    = context.Entry;
    public           RiftPlugin?    Instance { get; private set; }
    public           PluginContext  Context  => context;

    public Exception? Error { get; set; }

    public PluginStatus Status { get; set; }

    public bool Init()
    {
        if (_entry is null)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("No entry found."));
            return false;
        }

        if (_entry.GetTypes().FirstOrDefault(t => typeof(RiftPlugin).IsAssignableFrom(t) && !t.IsAbstract) is not
            { } type)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException($"Instance is not derived from <RiftPlugin>.\n  At: {_identity.EntryPath}"));
            Status = PluginStatus.Failed;

            return false;
        }

        if (FileVersionInfo.GetVersionInfo(_identity.EntryPath) is not { } info)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("Cannot get version info."));
            Status = PluginStatus.Failed;

            return false;
        }

        if (Activator.CreateInstance(type) is not RiftPlugin instance)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("Failed to create instance!"));
            Status = PluginStatus.Failed;
            return false;
        }


        Instance = instance;
        Status   = PluginStatus.Checked;

        return true;
    }

    public void Load()
    {
        try
        {
            if (Instance == null || !Instance.OnLoad())
            {

                throw new InvalidOperationException($"Failed to load plugin \"{_identity.EntryPath}\".");
            }

            if (Error != null)
            {
                throw Error;
            }
            Status = PluginStatus.Running;
        }
        catch (Exception e)
        {
            MakeError("An error occured when loading plugin.", e);
            // 出问题了，就得置空，不然就是野的
            Instance = null;
            Status   = PluginStatus.Failed;
        }
    }

    public void AllLoad()
    {
        Instance?.OnAllLoaded();
    }

    public void Unload(bool shutdown = false)
    {
        Instance?.OnUnload();
        
        // 如果没有错误, 那么就正常的把状态置空, 否则, 保存当前状态.
        if (Error is null)
        {
            Status = PluginStatus.None;
        }
        else
        {
            // 如果即将关闭shutdown(无论是关闭服务器还是module整个重新加载), 那么就无条件置空状态.
            if (shutdown)
            {
                Status = PluginStatus.None;
            }
        }

        Instance = null;
    }

    private void MakeError(string message, Exception e)
    {
        Error = e;
        bridge.PluginManager.Logger.LogError(Error, message);
    }
}