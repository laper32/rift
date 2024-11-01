// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Reflection;
using Rift.Runtime.API.Fundamental.Extensions;
using Rift.Runtime.API.Plugin;

namespace Rift.Runtime.Plugin;

// 其实这时候用Primary Constructor就很适合.

internal class PluginInstance(Assembly entry, string instancePath, string pluginPath)
{
    private const string DefaultReloadDirectory = "reload";
    public class PluginDescriptor(string instancePath, string pluginPath)
    {
        public string Name { get; set; } = "Unknown";
        public string Author { get; set; } = "Unknown";
        public string Version { get; set; } = "0.0.0.0";
        public string Url { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;

        public string Identifier => Path.GetFileNameWithoutExtension(pluginPath);
        public Guid UniqueId { get; set; }
        public string InstancePath => instancePath;
        public string PluginPath => pluginPath;
        public int ThreadId => Environment.CurrentManagedThreadId;
    }
    public PluginDescriptor Descriptor { get; init; } = new(instancePath, pluginPath);

    public RiftPlugin? Instance { get; private set; }

    public PluginStatus Status { get; set; }

    /// <summary>
    /// 按照提案, 如果运行时发生错误无法运行, 或者加载失败无法启动, 则应当附上为什么无法加载的错误信息,
    /// 方便后续排查. <br />
    /// 因此, 我们自然也需要提供一个Exception用于存放错误信息.
    /// </summary>
    public Exception? Error { get; set; }

    public bool Init()
    {
        if (entry.GetTypes().FirstOrDefault(t => typeof(RiftPlugin).IsAssignableFrom(t) && !t.IsAbstract) is not { } type)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException($"Instance is not derived from <RiftPlugin>.\n  At: {instancePath}"));
            Status = PluginStatus.Failed;

            return false;
        }

        if (FileVersionInfo.GetVersionInfo(Descriptor.InstancePath) is not { } info)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("Cannot get version info."));
            Status = PluginStatus.Failed;

            return false;
        }

        if (Attribute.GetCustomAttribute(entry, typeof(PluginAttribute)) is not PluginAttribute attr)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("Plugin metadata not found"));
            Status = PluginStatus.Failed;

            return false;
        }

        Descriptor.Url = attr.Url;
        Descriptor.Name = attr.Name;
        Descriptor.Author = attr.Author;
        Descriptor.Description = info.Comments ?? string.Empty;
        Descriptor.Version = info.ProductVersion!;


        if (Activator.CreateInstance(type) is not RiftPlugin instance)
        {
            MakeError("An error occured when loading plugin.", new BadImageFormatException("Shutdown to create instance!"));
            Status = PluginStatus.Failed;

            return false;
        }

        Descriptor.UniqueId = instance.UniqueId;
        type.SetPublicReadOnlyField("MyInfo", instance,
            new RiftPlugin.PluginInfo(
                Descriptor.Name,
                Descriptor.Author,
                Descriptor.Version,
                Descriptor.Url,
                Descriptor.Description
            )
        );

        type.BaseType!.SetReadOnlyField("_bridge", instance,
            new RiftPlugin.PluginInterfaceBridge(
                InstancePath: instancePath,
                RootPath: Descriptor.PluginPath
            )
        );

        Instance = instance;
        Status = PluginStatus.Checked;

        return true;
    }

    public void Load()
    {
        try
        {
            if (Instance == null || !Instance.OnLoad())
            {

                throw new InvalidOperationException($"Shutdown to load plugin \"{Descriptor.PluginPath}\".");
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
            Status = PluginStatus.Failed;
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

        //bridge.PluginManager.Logger.LogError(Error, message);
    }

    private string[]? GetUpdateFiles()
    {
        var reloadDir = Path.Combine(Descriptor.PluginPath, DefaultReloadDirectory);
        var sameNameDir = Path.Combine(Descriptor.PluginPath, Path.GetFileName(Descriptor.PluginPath));
        string[]? files = null;
        if (sameNameDir.Length > 0 && Directory.Exists(Path.Combine(Descriptor.PluginPath, sameNameDir)))
        {
            files = Directory.GetFiles(Path.Combine(Descriptor.PluginPath, sameNameDir));
        }
        else if (Directory.Exists(reloadDir))
        {
            files = Directory.GetFiles(reloadDir);
        }

        return files;
    }

    public bool IsUpdateRequired() => GetUpdateFiles() is { Length: > 0 };

    public void Update()
    {
        try
        {
            if (GetUpdateFiles() is not { } files) return;
            foreach (var file in files)
            {
                File.Copy(file, Path.Combine(Descriptor.PluginPath, Path.GetFileName(file)), true);
                File.Delete(file);
            }
        }
        catch (Exception e)
        {
            MakeError("An error occurred when updating plugin.", e);
        }
    }

    public void Cleanup()
    {
        try
        {
            var reloadDir = Path.Combine(Descriptor.PluginPath, DefaultReloadDirectory);
            var sameNameDir = Path.Combine(Descriptor.PluginPath, Path.GetFileName(Descriptor.PluginPath));
            if (Directory.Exists(reloadDir))
            {
                Directory.Delete(reloadDir, true);
            }

            if (Directory.Exists(sameNameDir))
            {
                Directory.Delete(sameNameDir, true);
            }
        }
        catch (Exception e)
        {
            MakeError("An error occured when cleaning up plugin.", e);
        }
    }
}

internal record PluginRuntimeInfo(string Name, string Author, string Version, string Url, string Description, string Path, string EntryPath, string Identifier, PluginStatus Status, Guid UniqueId, Exception? Error) : IPluginRuntimeInfo;
