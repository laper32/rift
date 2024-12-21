﻿using System.Reflection;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Modules.Abstractions;
using Rift.Runtime.Modules.Loader;

namespace Rift.Runtime.Modules.Fundamental;

internal class ModuleInstance(ModuleLoadContext context)
{
    private readonly Assembly       _entry    = context.Entry;
    private readonly ModuleIdentity _identity = context.Identity;

    public RiftModule?  Instance   { get; private set; }
    public Exception?   Error      { get; set; }
    public ModuleStatus Status     { get; set; }
    public ModuleType   ModuleType { get; init; } = context.Identity.ModuleType;

    public bool Init()
    {
        foreach (var t in _entry.GetTypes())
        {
            Console.WriteLine("Type Information: \n" +
                              $"  - Name: {t.FullName}\n" +
                              $"  - Derived From: {t.BaseType?.FullName}");
        }

        if (_entry.GetTypes().FirstOrDefault(t => typeof(RiftModule).IsAssignableFrom(t) && !t.IsAbstract) is not
            { } type)
        {
            MakeError("An error occured when loading module.",
                new BadImageFormatException(
                    $"Instance is not derived from {typeof(RiftModule)}.\n  At: {_identity.EntryPath}"));
            Status = ModuleStatus.Failed;
   
            return false;
        }

        if (Activator.CreateInstance(type) is not RiftModule instance)
        {
            MakeError("An error occured when loading module.",
                new BadImageFormatException("Failed to create instance!"));
            Status = ModuleStatus.Failed;
            return false;
        }
           
        Instance = instance;
        Status   = ModuleStatus.Checked;

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
   
            Status = ModuleStatus.Running;
        }
        catch (Exception e)
        {
            MakeError("An error occured when loading module.", e);
            // 出问题了，就得置空，不然就是野的
            Instance = null;
            Status   = ModuleStatus.Failed;
        }
    }

    public void Unload(bool shutdown = false)
    {
        Instance?.OnUnload();
   
        // 如果没有错误, 那么就正常的把状态置空, 否则, 保存当前状态.
        if (Error is null)
        {
            Status = ModuleStatus.None;
        }
        else
        {
            // 如果即将关闭shutdown(无论是关闭服务器还是module整个重新加载), 那么就无条件置空状态.
            if (shutdown)
            {
                Status = ModuleStatus.None;
            }
        }
   
        Instance = null;
    }

    public void PostLoad()
    {
        Instance?.OnAllLoaded();
    }

    private void MakeError(string message, Exception e)
    {
        Error = e;
        Tty.Error($"{message} ({e.Message})");
    }
}
