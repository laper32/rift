// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Schema;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Task;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Manifest;
using Rift.Runtime.Plugin;
using Rift.Runtime.Scripting;
using Tomlyn;

namespace Rift.Runtime.Workspace;

internal interface IWorkspaceManagerInternal : IWorkspaceManager
{
    void LoadWorkspace();

    void SetRootPath(string path);

    bool AddMetadataForPackage(string key, object value);
    bool AddDependencyForPackage(IPackageImportDeclarator declarator);
    bool AddDependencyForPackage(IEnumerable<IPackageImportDeclarator> declarators);

    bool AddPluginForPackage(Scripting.Plugin plugin);
    bool AddPluginForPackage(IEnumerable<Scripting.Plugin> plugins);

    IEnumerable<PluginDescriptor> CollectPluginsForLoad();
}

internal class WorkspaceManager : IWorkspaceManagerInternal, IInitializable
{

    private readonly Packages _packages = new();
    internal readonly PackageInstances PackageInstances = new();

    private EWorkspaceStatus _status;
    public string           Root { get; internal set; }

    public WorkspaceManager()
    {
        Root = "__Unknown__";
        _status = EWorkspaceStatus.Unknown;
        IWorkspaceManager.Instance = this;
    }

    public bool Init()
    {
        _status = EWorkspaceStatus.Init;
        return true;
    }

    public void Shutdown()
    {
        _status = EWorkspaceStatus.Shutdown;
    }

    #region Fundamental operations

    public void SetRootPath(string path)
    {
        // NOTE: 现在只考虑根目录的情况，不考虑从下往上搜的情况（因为从下到上需要带Context。）
        // 现在我们没办法处理这个问题，得先自顶向下正确解析了才能处理自底向上的问题。
        var rootManifest = GetRootManifest(path);
        if (rootManifest.EndsWith(Definitions.ManifestIdentifier))
        {
            rootManifest = Path.GetDirectoryName(rootManifest)!;
        }

        Root = rootManifest;
    }

    #endregion

    public void LoadWorkspace()
    {
        var manifestPath = Path.Combine(Root, Definitions.ManifestIdentifier);
        _packages.LoadRecursively(manifestPath);
        ValidateWorkspace();
        ActivatePackage();
        //PackageInstances.DumpInstancesMetadata();
        _status = EWorkspaceStatus.Ready;
    }

    public void ValidateWorkspace()
    {
        // TODO: 检查是否存在脚本错误引用的问题
        // TODO:    包括但不限于:
        // TODO:        多个field引用同一个脚本
        // TODO:        field引用非自身包的脚本
    }

    public void ActivatePackage()
    {
        foreach (var (packageName, maybePackage) in _packages.Value)
        {
            PackageInstances.Add(packageName, new PackageInstance(maybePackage));
        }

        EvaluateManifestScripts();
    }

    #region Manifest operations

    public static TomlManifest LoadManifest(string path)
    {
        var text = File.ReadAllText(path);
        var content = Toml.ToModel<TomlManifest>(text, options: new TomlModelOptions
        {
            IgnoreMissingProperties = true
        });
        return content;
    }

    public static IEitherManifest ReadManifest(string path)
    {
        var schema = LoadManifest(path);
        if (schema is null)
        {
            throw new InvalidOperationException($"Shutdown to load manifest from `{path}`");
        }

        if (schema.Workspace is { } workspace)
        {
            if (schema.Folder is not null || schema.Project is not null || schema.Target is not null || schema.Plugin is not null)
            {
                throw new InvalidOperationException("Workspace and Folder/Project/Target/Plugin can't be used together.");
            }

            var workspaceName = "";
            var schemaName = workspace.Name;

            if (schemaName != null)
            {
                workspaceName = schemaName;
            }
            else
            {
                var manifestLocation = Path.GetDirectoryName(path)!;
                var workspaceRoot = IWorkspaceManager.Instance.Root;
                if (workspaceRoot.Equals(manifestLocation))
                {
                    workspaceName = Path.GetFileName(manifestLocation);
                }
            }

            // ReSharper disable once InvertIf
            if (schema.Task is { } tasks)
            {
                var taskManifests = MakeTaskManifests(tasks);
                ITaskManager.Instance.RegisterTask(workspaceName, taskManifests);
            }

            return new EitherManifest<VirtualManifest<WorkspaceManifest>>(
                new VirtualManifest<WorkspaceManifest>(
                    new WorkspaceManifest(
                        Name: workspaceName,
                        Members: workspace.Members ?? [],
                        Exclude: workspace.Exclude ?? [],
                        Plugins: workspace.Plugins,
                        Dependencies: workspace.Dependencies,
                        Configure: workspace.Configure
                    )
                )
            );
        }

        if (schema.Folder is { } folder)
        {
            if (schema.Workspace is not null || schema.Project is not null || schema.Target is not null || schema.Plugin is not null)
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

                var workspaceRoot = IWorkspaceManager.Instance.Root;
                if (workspaceRoot.Equals(manifestLocation))
                {
                    folderName = Path.GetFileName(manifestLocation);
                }
            }

            return new EitherManifest<VirtualManifest<FolderManifest>>(
                new VirtualManifest<FolderManifest>(
                    new FolderManifest(
                        Name: folderName,
                        Members: schema.Folder.Members ?? [],
                        Exclude: schema.Folder.Exclude ?? []
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
                    throw new InvalidOperationException("`project.members` and `project.exclude` cannot occur when `[target]` field exists.");
                }

                //if (sameLayeredTarget.Dependencies is not null)
                //{
                //    Console.WriteLine($"Warning: Target `{sameLayeredTarget.Name}`'s dependency script will be shadowed, because it is at the same layer of the project `{schema.Project.Name}`.");
                //}

                //if (sameLayeredTarget.Plugins is not null)
                //{
                //    Console.WriteLine($"Warning: Target `{sameLayeredTarget.Name}`'s plugins script will be shadowed, because it is at the same layer of the project `{schema.Project.Name}`.");
                //}

                //if (sameLayeredTarget.Configure is not null)
                //{
                //    Console.WriteLine($"Warning: Target `{sameLayeredTarget.Name}`'s metadata script will be shadowed, because it is at the same layer of the project `{schema.Project.Name}`.");
                //}

                var targetManifest = new TargetManifest(
                    Name: sameLayeredTarget.Name,
                    Type: sameLayeredTarget.Type,
                    Dependencies: null,
                    Configure: null,
                    Plugins: null
                );

                var projectManifest = new ProjectManifest(
                    Name: project.Name,
                    Authors: project.Authors,
                    Version: project.Version,
                    Description: project.Description ?? string.Empty,
                    Dependencies: project.Dependencies,
                    Configure: project.Configure,
                    Plugins: project.Plugins,
                    Target: targetManifest,
                    Members: null,
                    Exclude: null
                );

                // ReSharper disable once InvertIf
                if (schema.Task is { } tasks)
                {
                    var taskManifests = MakeTaskManifests(tasks);
                    ITaskManager.Instance.RegisterTask(project.Name, taskManifests);
                }

                var manifest = new Manifest<ProjectManifest>(projectManifest);
                var ret = new EitherManifest<Manifest<ProjectManifest>>(manifest);
                return ret;
            }
            else
            {
                var projectMembers = project.Members ?? [];
                var projectExclude = project.Exclude ?? [];
                var projectManifest = new ProjectManifest(
                    Name: project.Name,
                    Authors: project.Authors,
                    Version: project.Version,
                    Description: project.Description ?? string.Empty,
                    Dependencies: project.Dependencies,
                    Configure: project.Configure,
                    Plugins: project.Plugins,
                    Target: null,
                    Members: projectMembers,
                    Exclude: projectExclude
                );

                // ReSharper disable once InvertIf
                if (schema.Task is { } tasks)
                {
                    var taskManifests = MakeTaskManifests(tasks);
                    ITaskManager.Instance.RegisterTask(project.Name, taskManifests);
                }

                var manifest = new Manifest<ProjectManifest>(projectManifest);
                var ret = new EitherManifest<Manifest<ProjectManifest>>(manifest);
                return ret;
            }
        }

        if (schema.Target is { } target)
        {
            if (schema.Folder is not null || schema.Workspace is not null || schema.Plugin is not null)
            {
                throw new InvalidOperationException("Target cannot used together with `[workspace]`, `[folder]`, or `[plugin]`");
            }

            // ReSharper disable once InvertIf
            if (schema.Task is { } tasks)
            {
                var taskManifests = MakeTaskManifests(tasks);
                ITaskManager.Instance.RegisterTask(target.Name, taskManifests);
            }

            return new EitherManifest<Manifest<TargetManifest>>(
                new Manifest<TargetManifest>(
                    new TargetManifest(
                        Name: target.Name,
                        Type: target.Type,
                        Plugins: target.Plugins,
                        Dependencies: target.Dependencies,
                        Configure: target.Configure
                    )
                )
            );
        }

        // ReSharper disable once InvertIf
        if (schema.Plugin is { } plugin)
        {
            if (schema.Folder is not null || schema.Workspace is not null || schema.Project is not null || schema.Target is not null)
            {
                throw new InvalidOperationException("Plugin cannot used together with `[workspace]`, `[folder]`, `[project]`, or `[target]`");
            }

            // ReSharper disable once InvertIf
            if (schema.Task is { } tasks)
            {
                var taskManifests = MakeTaskManifests(tasks);
                ITaskManager.Instance.RegisterTask(plugin.Name, taskManifests);
            }


            return new EitherManifest<RiftManifest<PluginManifest>>(
                new RiftManifest<PluginManifest>(
                    new PluginManifest(
                        Name: plugin.Name,
                        Authors: plugin.Authors,
                        Version: plugin.Version,
                        Description: plugin.Description ?? string.Empty,
                        Configure: plugin.Configure,
                        Dependency: plugin.Dependencies
                    )
                )
            );
        }

        throw new InvalidOperationException($"No any workspace schema field found, path: `{path}`");
    }

    private static List<TaskManifest> MakeTaskManifests(Dictionary<string, TomlTask> tasks)
    {
        var taskManifests = new List<TaskManifest>();
        tasks.ForEach((taskName, taskToml) =>
        {
            List<TaskArgManifest>? taskArgs = null;
            if (taskToml.Args is { } argsToml)
            {
                taskArgs = new List<TaskArgManifest>();
                argsToml.ForEach(x =>
                {
                    taskArgs.Add(new TaskArgManifest(
                        Name: x.Name,
                        Short: x.Short,
                        Description: x.Description,
                        Default: x.Default,
                        ConflictWith: x.ConflictWith,
                        Heading: x.Heading));
                });
            }

            taskManifests.Add(
                new TaskManifest(
                    Name: taskName,
                    IsCommand: taskToml.IsCommand,
                    Heading: taskToml.Heading,
                    BeforeHelp: taskToml.BeforeHelp,
                    AfterHelp: taskToml.AfterHelp,
                    Description: taskToml.About,
                    Parent: taskToml.Parent,
                    SubTasks: taskToml.SubTasks,
                    RunTasks: taskToml.RunTasks,
                    Args: taskArgs
                )
            );
        });
        return taskManifests;
    }

    /// <summary>
    /// 计算脚本路径是基于传入的Manifest路径判断的。 <br/>
    /// 此时传入的Manifest路径一定带有Rift.toml
    /// </summary>
    /// <param name="manifestPath">Manifest路径</param>
    /// <param name="scriptPath">脚本路径</param>
    /// <returns></returns>
    public static string GetActualScriptPath(string manifestPath, string scriptPath)
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
        const string invalidManifestName = "rift.toml";
        var hasInvalidManifestPath = false;
        var current = new DirectoryInfo(cwd);
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
            throw new Exception($"could not find `{Definitions.ManifestIdentifier}` in `{cwd}` or any parent directory, but found {invalidManifestName} please try to rename it to {Definitions.ManifestIdentifier}");
        }

        throw new Exception(
            $"could not find `{Definitions.ManifestIdentifier}` in `{cwd}` or any parent directory.");
    }

    #endregion

    #region Scripts operations

    private void EvaluateManifestScripts()
    {
        RetrieveWorkspacePlugins();
        RetrieveWorkspaceDependencies();
        RetrieveWorkspaceMetadata();
    }

    private void RetrieveWorkspaceDependencies()
    {
        foreach (var package in _packages.Value)
        {
            if (package.Value.Dependencies is not { } dependencies)
            {
                continue;
            }
            IScriptManager.Instance.EvaluateScript(dependencies);
        }
    }

    private void RetrieveWorkspaceMetadata()
    {
        foreach (var package in _packages.Value)
        {
            if (package.Value.Metadata is not { } metadata)
            {
                continue;
            }

            IScriptManager.Instance.EvaluateScript(metadata);
        }
    }

    private void RetrieveWorkspacePlugins()
    {
        foreach (var package in _packages.Value)
        {
            if (package.Value.Plugins is not { } plugins)
            {
                continue;
            }

            IScriptManager.Instance.EvaluateScript(plugins);
        }
    }

    public bool AddMetadataForPackage(string key, object value)
    {
        if (GetPackageInstance() is not { } instance)
        {
            return false;
        }
        instance.Metadata.Add(key, value);
        return true;

    }

    public bool AddDependencyForPackage(IPackageImportDeclarator declarator)
    {
        if (GetPackageInstance() is not { } instance)
        {
            return false;
        }
        instance.Dependencies.Add(declarator.Name, declarator);
        return true;

    }

    public bool AddDependencyForPackage(IEnumerable<IPackageImportDeclarator> declarators)
    {
        if (GetPackageInstance() is not { } packageInstance)
        {
            return false;
        }

        foreach (var declarator in declarators)
        {
            packageInstance.Dependencies.Add(declarator.Name, declarator);
        }

        return true;
    }

    public bool AddPluginForPackage(Scripting.Plugin plugin)
    {
        if (GetPackageInstance() is not { } packageInstance)
        {
            return false;
        }

        packageInstance.Plugins.Add(plugin.Name, plugin);
        return true;
    }

    public bool AddPluginForPackage(IEnumerable<Scripting.Plugin> plugins)
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
        var scriptSystem = (IScriptManagerInternal)IScriptManager.Instance;
        if (scriptSystem.ScriptContext is not { } scriptContext)
        {
            throw new InvalidOperationException("This function is only allowed in package dependency script.");
        }

        return PackageInstances.FindPackageFromScriptPath(scriptContext.Path);
    }

    #endregion

    public IEnumerable<PluginDescriptor> CollectPluginsForLoad()
    {
        CheckAvailable();
        return PackageInstances.CollectPluginsForLoad();
    }

    private void CheckAvailable()
    {
        if (_status is not (EWorkspaceStatus.Ready or EWorkspaceStatus.Init))
        {
            throw new InvalidOperationException("WorkspaceManager is not available.");
        }
    }
}