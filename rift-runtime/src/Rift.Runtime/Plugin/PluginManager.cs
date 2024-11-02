// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Plugin;

internal interface IPluginManagerInternal : IPluginManager, IInitializable
{
    void LoadPlugins();
}

internal class PluginManager : IPluginManagerInternal
{
    private const string PluginDirectoryName    = "plugins";
    private       string _installationPluginPath = Path.Combine(IRuntime.Instance.InstallationPath, PluginDirectoryName);
    private       string _userPluginPath         = Path.Combine(IRuntime.Instance.UserPath, PluginDirectoryName);

    public PluginManager()
    {
        IPluginManager.Instance = this;
    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {

    }

    public void LoadPlugins()
    {
        var workspaceManager    = (IWorkspaceManagerInternal) IWorkspaceManager.Instance;
        var declarators = workspaceManager.CollectPluginsForLoad();
        var result = JsonSerializer.Serialize(declarators, new JsonSerializerOptions
        {
            WriteIndented = true
        });
        Console.WriteLine(result);
    }

    private void EnumerateExistingPlugins()
    {
    }

    
}