// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;
using Rift.Runtime.Collections.Generic;
using Rift.Runtime.Fundamental;
using Rift.Runtime.Manifest;
using Rift.Runtime.Plugins;
using Rift.Runtime.Schema;
using Rift.Runtime.Scripting;
using Tomlyn;

namespace Rift.Runtime.Workspace;

public sealed class WorkspaceManager
{
    private static   WorkspaceManager         _instance  = null!;
    private readonly List<PluginListenerInfo> _listeners = [];
    private readonly PackageInstances         _packageInstances;
    private readonly Packages                 _packages;

    private EWorkspaceStatus _status;

    public WorkspaceManager()
    {
        _status           = EWorkspaceStatus.Unknown;
        _packages         = new Packages();
        _packageInstances = new PackageInstances();
        _instance         = this;
    }

    public string Root { get; private set; } = "__Unknown__";

    internal static bool Init()
    {
        _instance._status          =  EWorkspaceStatus.Init;
        PluginManager.PluginUnload += _instance.OnPluginUnload;
        ScriptManager.AddNamespace("Rift.Runtime.Workspace");
        return true;
    }

    internal static void Shutdown()
    {
        PluginManager.PluginUnload -= _instance.OnPluginUnload;
        ScriptManager.RemoveNamespace("Rift.Runtime.Workspace");
        _instance._status = EWorkspaceStatus.Shutdown;
    }

    internal static void LoadWorkspace()
    {
        var manifestPath = Path.Combine(_instance.Root, Definitions.ManifestIdentifier);
        _instance._packages.LoadRecursively(manifestPath);
        _instance.ValidateWorkspace();
        _instance.ActivatePackage();

        _instance._status = EWorkspaceStatus.Ready;
    }

    internal void ValidateWorkspace()
    {
        // TODO: 检查是否存在脚本错误引用的问题
        // TODO:    包括但不限于:
        // TODO:        多个field引用同一个脚本
        // TODO:        field引用非自身包的脚本
    }

    #region Fundamental operations

    internal static void SetRootPath(string path)
    {
        // NOTE: 现在只考虑根目录的情况，不考虑从下往上搜的情况（因为从下到上需要带Context。）
        // 现在我们没办法处理这个问题，得先自顶向下正确解析了才能处理自底向上的问题。
        var rootManifest = _instance.GetRootManifest(path);
        if (rootManifest.EndsWith(Definitions.ManifestIdentifier))
        {
            rootManifest = Path.GetDirectoryName(rootManifest)!;
        }

        _instance.Root = rootManifest;
    }

    #endregion

    #region PluginManager

    private void OnPluginUnload(PluginInstance instance)
    {
        if (instance.Instance is not { } internalInstance)
        {
            return;
        }

        var listeners = _listeners.FindAll(x => x.Instance == internalInstance).ToArray();
        foreach (var info in listeners)
        {
            RemoveListener(info.Instance, info.Listener);
        }
    }

    #endregion

    private record PluginListenerInfo(RiftPlugin Instance, IWorkspaceListener Listener);

    #region Package Operations

    internal void ActivatePackage()
    {
        foreach (var (packageName, maybePackage) in _packages.Value)
        {
            _packageInstances.Add(packageName, new PackageInstance(maybePackage));
        }

        RunWorkspacePluginsScript();

        PluginManager.NotifyLoadPlugins();

        RunWorkspaceDependenciesScript();
        RunWorkspaceConfigurationScript();

        OnAllPackageLoaded();
    }

    public static IPackageInstance? FindPackage(string name)
    {
        return _instance._packageInstances.FindInstance(name);
    }

    public static IEnumerable<IPackageInstance> GetAllPackages()
    {
        return _instance._packageInstances.GetAllInstances();
    }

    internal static IEnumerable<PluginDescriptor> CollectPluginsForLoad()
    {
        _instance.CheckAvailable();

        return _instance._packageInstances.CollectPluginsForLoad();
    }

    private void CheckAvailable()
    {
        if (_status is not (EWorkspaceStatus.Ready or EWorkspaceStatus.Init))
        {
            throw new InvalidOperationException("WorkspaceManager is not available.");
        }
    }

    #endregion

    #region Listeners Operation

    /// <summary>
    ///     Adding a listener <br />
    ///     Plugin system is unnecessary to consider when remove the listener, PluginManager will automatically remove it when
    ///     plugin unloads.
    /// </summary>
    /// <param name="instance"> Plugin instance. </param>
    /// <param name="listener"> Workspace Listener </param>
    public static void AddListener(RiftPlugin instance, IWorkspaceListener listener)
    {
        if (_instance._listeners.Find(x => x.Instance == instance && x.Listener == listener) != null)
        {
            Tty.Warning($"You have already installed same listener!{Environment.NewLine}{Environment.StackTrace}");
            return;
        }

        _instance._listeners.Add(new PluginListenerInfo(instance, listener));
    }

    public static void RemoveListener(RiftPlugin instance, IWorkspaceListener listener)
    {
        if (_instance._listeners.Find(x => x.Instance == instance && x.Listener == listener) is not { } data)
        {
            return;
        }

        _instance._listeners.Remove(data);
    }

    private void OnAllPackageLoaded()
    {
        _listeners.ForEachCatchException(x => x.Listener.OnAllPackagesLoaded(),
            e => { Tty.Error(e, $"An error occured when calling {nameof(OnAllPackageLoaded)}"); });
    }

    #endregion

    #region Manifest operations

    internal static TomlManifest LoadManifest(string path)
    {
        var text      = File.ReadAllText(path);
        var tomlModel = Toml.ToModel(text);
        var ret = JsonSerializer.Deserialize<TomlManifest>(JsonSerializer.Serialize(tomlModel)) ??
                  throw new InvalidOperationException("Failed to load manifest.");

        return ret;
    }

    internal static IEitherManifest ReadManifest(string path)
    {
        var schema = LoadManifest(path);
        if (schema is null)
        {
            throw new InvalidOperationException($"Shutdown to load manifest from `{path}`");
        }

        if (schema.Workspace is { } workspace)
        {
            if (schema.Folder is not null || schema.Project is not null || schema.Target is not null ||
                schema.Plugin is not null)
            {
                throw new InvalidOperationException(
                    "Workspace and Folder/Project/Target/Plugin can't be used together.");
            }

            var workspaceName = "";
            var schemaName    = workspace.Name;

            if (schemaName != null)
            {
                workspaceName = schemaName;
            }
            else
            {
                var manifestLocation = Path.GetDirectoryName(path)!;
                var workspaceRoot    = _instance.Root;
                if (workspaceRoot.Equals(manifestLocation))
                {
                    workspaceName = Path.GetFileName(manifestLocation);
                }
            }

            return new EitherManifest<VirtualManifest<WorkspaceManifest>>(
                new VirtualManifest<WorkspaceManifest>(
                    new WorkspaceManifest(
                        workspaceName,
                        workspace.Members ?? [],
                        workspace.Exclude ?? [],
                        workspace.Plugins,
                        Dependencies: workspace.Dependencies,
                        Configure: workspace.Configure,
                        Others: workspace.Others
                    )
                )
            );
        }

        if (schema.Folder is { } folder)
        {
            if (schema.Workspace is not null || schema.Project is not null || schema.Target is not null ||
                schema.Plugin is not null)
            {
                throw new InvalidOperationException(
                    "Workspace and Folder/Project/Target/Plugin can't be used together.");
            }

            var folderName = "";
            var schemaName = folder.Name;

            if (schemaName != null)
            {
                folderName = schemaName;
            }
            else
            {
                var manifestLocation = Path.GetDirectoryName(path)!;

                var workspaceRoot = _instance.Root;
                if (workspaceRoot.Equals(manifestLocation))
                {
                    folderName = Path.GetFileName(manifestLocation);
                }
            }

            return new EitherManifest<VirtualManifest<FolderManifest>>(
                new VirtualManifest<FolderManifest>(
                    new FolderManifest(
                        folderName,
                        schema.Folder.Members ?? [],
                        schema.Folder.Exclude ?? []
                    )
                )
            );
        }

        if (schema.Project is { } project)
        {
            if (schema.Folder is not null || schema.Workspace is not null || schema.Plugin is not null)
            {
                throw new InvalidOperationException("Workspace and Folder/Project/Plugin can't be used together.");
            }

            // 如果此时Project和Target在同一级，此时Target的脚本文件将会被直接无视，只看project级别的脚本文件。
            var sameLayeredTarget = schema.Target;

            if (sameLayeredTarget is not null)
            {
                if (project.Members is not null || project.Exclude is not null)
                {
                    throw new InvalidOperationException(
                        "`project.members` and `project.exclude` cannot occur when `[target]` field exists.");
                }

                // NOTE: 需要做额外检查吗？比如说A包的脚本路径和在B包里面之类的

                var targetManifest = new TargetManifest(
                    sameLayeredTarget.Name,
                    sameLayeredTarget.Type,
                    Dependencies: null,
                    Configure: null,
                    Plugins: null,
                    Others: sameLayeredTarget.Others
                );

                var projectManifest = new ProjectManifest(
                    project.Name,
                    project.Authors,
                    project.Version,
                    project.Description ?? string.Empty,
                    Dependencies: project.Dependencies,
                    Configure: project.Configure,
                    Plugins: project.Plugins,
                    Target: targetManifest,
                    Members: null,
                    Exclude: null,
                    Others: project.Others
                );

                var manifest = new Manifest<ProjectManifest>(projectManifest);
                var ret      = new EitherManifest<Manifest<ProjectManifest>>(manifest);
                return ret;
            }
            else
            {
                var projectMembers = project.Members ?? [];
                var projectExclude = project.Exclude ?? [];
                var projectManifest = new ProjectManifest(
                    project.Name,
                    project.Authors,
                    project.Version,
                    project.Description ?? string.Empty,
                    Dependencies: project.Dependencies,
                    Configure: project.Configure,
                    Plugins: project.Plugins,
                    Target: null,
                    Members: projectMembers,
                    Exclude: projectExclude,
                    Others: project.Others
                );

                var manifest = new Manifest<ProjectManifest>(projectManifest);
                var ret      = new EitherManifest<Manifest<ProjectManifest>>(manifest);
                return ret;
            }
        }

        if (schema.Target is { } target)
        {
            if (schema.Folder is not null || schema.Workspace is not null || schema.Plugin is not null)
            {
                throw new InvalidOperationException(
                    "Target cannot used together with `[workspace]`, `[folder]`, or `[plugin]`");
            }

            return new EitherManifest<Manifest<TargetManifest>>(
                new Manifest<TargetManifest>(
                    new TargetManifest(
                        target.Name,
                        target.Type,
                        target.Plugins,
                        target.Dependencies,
                        target.Configure,
                        target.Others
                    )
                )
            );
        }

        // ReSharper disable once InvertIf
        if (schema.Plugin is { } plugin)
        {
            if (schema.Folder is not null || schema.Workspace is not null || schema.Project is not null ||
                schema.Target is not null)
            {
                throw new InvalidOperationException(
                    "Plugin cannot used together with `[workspace]`, `[folder]`, `[project]`, or `[target]`");
            }

            return new EitherManifest<RiftManifest<PluginManifest>>(
                new RiftManifest<PluginManifest>(
                    new PluginManifest(
                        plugin.Name,
                        plugin.Authors,
                        plugin.Version,
                        plugin.Description ?? string.Empty,
                        plugin.Configure,
                        plugin.Dependencies,
                        plugin.Others
                    )
                )
            );
        }

        throw new InvalidOperationException($"No any workspace schema field found, path: `{path}`");
    }

    /// <summary>
    ///     计算脚本路径是基于传入的Manifest路径判断的。 <br />
    ///     此时传入的Manifest路径一定带有Rift.toml
    /// </summary>
    /// <param name="manifestPath"> Manifest路径 </param>
    /// <param name="scriptPath"> 脚本路径 </param>
    /// <returns> </returns>
    internal static string GetActualScriptPath(string manifestPath, string scriptPath)
    {
        if (!manifestPath.EndsWith("Rift.toml"))
        {
            throw new InvalidOperationException("No `Rift.toml` found.");
        }


        return Path.Combine(Path.GetDirectoryName(manifestPath)!, scriptPath);
    }

    private string GetRootManifest(string manifestPath)
    {
        var path = Path.Combine(Environment.CurrentDirectory, manifestPath);
        if (!path.EndsWith(Definitions.ManifestIdentifier))
        {
            return FindRootManifestForCurrentDir(path);
        }

        if (!Path.Exists(path))
        {
            throw new Exception($"manifest path `{manifestPath}` does not exist");
        }

        return path;
    }

    private string FindRootManifestForCurrentDir(string cwd)
    {
        const string invalidManifestName    = "rift.toml";
        var          hasInvalidManifestPath = false;
        var          current                = new DirectoryInfo(cwd);
        while (current != null)
        {
            var validManifest = Path.Combine(current.FullName, Definitions.ManifestIdentifier);
            if (File.Exists(validManifest))
            {
                return validManifest;
            }

            var invalidManifest = Path.Combine(Path.Combine(current.FullName, invalidManifestName));
            if (File.Exists(invalidManifest))
            {
                hasInvalidManifestPath = true;
            }

            current = current.Parent;
        }

        if (hasInvalidManifestPath)
        {
            throw new Exception(
                $"could not find `{Definitions.ManifestIdentifier}` in `{cwd}` or any parent directory, but found {invalidManifestName} please try to rename it to {Definitions.ManifestIdentifier}");
        }

        throw new Exception(
            $"could not find `{Definitions.ManifestIdentifier}` in `{cwd}` or any parent directory.");
    }

    #endregion

    #region Scripts operations

    private void RunWorkspaceDependenciesScript()
    {
        _packageInstances.ForEach((_, instance) =>
        {
            if (instance.Value.Dependencies is not { } dependencies)
            {
                return;
            }

            ScriptManager.EvaluateScript(dependencies);
        });
    }

    private void RunWorkspaceConfigurationScript()
    {
        _packageInstances.ForEach((_, instance) =>
        {
            if (instance.Value.Configure is not { } configure)
            {
                return;
            }

            ScriptManager.EvaluateScript(configure);
        });
    }

    private void RunWorkspacePluginsScript()
    {
        _packageInstances.ForEach((_, instance) =>
        {
            if (instance.Value.Plugins is not { } plugins)
            {
                return;
            }

            ScriptManager.EvaluateScript(plugins);
        });
    }

    internal static void ConfigurePackage(Action<PackageConfiguration> predicate)
    {
        _instance.ConfigurePackageCanonicalized(predicate);
    }

    private void ConfigurePackageCanonicalized(Action<PackageConfiguration> predicate)
    {
        if (GetPackageInstance() is not { } instance)
        {
            return;
        }

        var configuration = new PackageConfiguration();
        predicate(configuration);
        configuration.Attributes.ForEach(kv => { instance.Configuration.Attributes.Add(kv.Key, kv.Value); });
    }

    internal static bool AddDependencyForPackage(PackageReference reference)
    {
        return _instance.AddDependencyForPackageCanonicalized([reference]);
    }

    internal static bool AddDependencyForPackage(IEnumerable<PackageReference> references)
    {
        return _instance.AddDependencyForPackageCanonicalized(references);
    }

    private bool AddDependencyForPackageCanonicalized(IEnumerable<PackageReference> references)
    {
        if (GetPackageInstance() is not { } instance)
        {
            return false;
        }
        foreach (var reference in references)
        {
            instance.Dependencies.Add(reference.Name, reference);
        }

        return true;
    }

    internal static bool AddPluginForPackage(PackageReference plugin)
    {
        return _instance.AddPluginForPackageCanonicalized([plugin]);
    }

    internal static bool AddPluginForPackage(IEnumerable<PackageReference> plugins)
    {
        return _instance.AddPluginForPackageCanonicalized(plugins);
    }

    private bool AddPluginForPackageCanonicalized(IEnumerable<PackageReference> plugins)
    {
        if (GetPackageInstance() is not { } packageInstance)
        {
            return false;
        }

        foreach (var plugin in plugins)
        {
            packageInstance.Plugins.Add(plugin.Name, plugin);
        }

        return true;
    }

    private PackageInstance? GetPackageInstance()
    {
        if (ScriptManager.ScriptContext is not { } scriptContext)
        {
            throw new InvalidOperationException("This function is only allowed in package dependency script.");
        }

        return _packageInstances.FindPackageFromScriptPath(scriptContext.Path);
    }

    #endregion
}