// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Schema;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Manifest;
using Tomlyn;

namespace Rift.Runtime.Workspace;

internal interface IWorkspaceManagerInternal : IWorkspaceManager, IInitializable
{
    public EWorkspaceStatus Status { get; }

    void LoadWorkspace();

    void SetRootPath(string path);
}

internal class WorkspaceManager : IWorkspaceManagerInternal
{
    private readonly Packages _packages = new();

    public EWorkspaceStatus Status { get; internal set; }
    public string Root { get; internal set; }

    public WorkspaceManager()
    {
        Root = "__Unknown__";
        Status = EWorkspaceStatus.Unknown;
        IWorkspaceManager.Instance = this;
    }

    public bool Init()
    {
        Status = EWorkspaceStatus.Init;
        return true;
    }

    public void Shutdown()
    {
        Status = EWorkspaceStatus.Unknown;
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
        _packages.LoadRecursively(Path.Combine(Root, Definitions.ManifestIdentifier));
        EvaluateManifestScripts();
    }

    public void PrintMessage()
    {
        Console.WriteLine("Invoked.");
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
            throw new InvalidOperationException($"Failed to load manifest from `{path}`");
        }

        if (schema.Workspace is { } workspace)
        {
            if (schema.Folder is not null || schema.Project is not null || schema.Target is not null)
            {
                throw new InvalidOperationException("Workspace and Folder/Project/Target can't be used together.");
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

            return new EitherManifest<VirtualManifest<WorkspaceManifest>>(
                new VirtualManifest<WorkspaceManifest>(
                    new WorkspaceManifest(
                        Name: workspaceName,
                        Members: workspace.Members ?? [],
                        Exclude: workspace.Exclude ?? [],
                        Plugins: workspace.Plugins,
                        Dependencies: workspace.Dependencies,
                        Metadata: workspace.Metadata
                    )
                )
            );
        }

        if (schema.Folder is { } folder)
        {
            if (schema.Workspace is not null || schema.Project is not null || schema.Target is not null)
            {
                throw new InvalidOperationException(
                    "Workspace and Folder/Project/Target can't be used together.");
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
            if (schema.Folder is not null || schema.Workspace is not null)
            {
                throw new InvalidOperationException("Workspace and Folder/Project/Plugin can't be used together.");
            }

            var sameLayeredTarget = schema.Target;

            if (sameLayeredTarget is not null)
            {
                if (project.Members is not null || project.Exclude is not null)
                {
                    throw new InvalidOperationException("`project.members` and `project.exclude` cannot occur when `[target]` field exists.");
                }

                var targetManifest = new TargetManifest(
                    Name: sameLayeredTarget.Name,
                    Type: sameLayeredTarget.Type,
                    Dependencies: sameLayeredTarget.Dependencies,
                    Metadata: sameLayeredTarget.Metadata,
                    Plugins: sameLayeredTarget.Plugins
                );

                var projectManifest = new ProjectManifest(
                    Name: project.Name,
                    Authors: project.Authors,
                    Version: project.Version,
                    Description: project.Description ?? string.Empty,
                    Dependencies: project.Dependencies,
                    Metadata: project.Metadata,
                    Plugins: project.Plugins,
                    Target: targetManifest,
                    Members: null,
                    Exclude: null
                );
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
                    Metadata: project.Metadata,
                    Plugins: project.Plugins,
                    Target: null,
                    Members: projectMembers,
                    Exclude: projectExclude
                );
                var manifest = new Manifest<ProjectManifest>(projectManifest);
                var ret = new EitherManifest<Manifest<ProjectManifest>>(manifest);
                return ret;
            }
        }

        // ReSharper disable once InvertIf
        if (schema.Target is { } target)
        {
            if (schema.Folder is not null || schema.Workspace is not null)
            {
                throw new InvalidOperationException("Target cannot used together with `[workspace]` or `[folder0000]`");
            }

            return new EitherManifest<Manifest<TargetManifest>>(
                new Manifest<TargetManifest>(
                    new TargetManifest(
                        Name: target.Name,
                        Type: target.Type,
                        Plugins: target.Plugins,
                        Dependencies: target.Dependencies,
                        Metadata: target.Metadata
                    )
                )
            );

        }

        throw new InvalidOperationException($"No any workspace schema field found, path: `{path}`");
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
            IScriptSystem.Instance.EvaluateScript(dependencies);
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

            IScriptSystem.Instance.EvaluateScript(metadata);
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

            IScriptSystem.Instance.EvaluateScript(plugins);
        }
    }

    #endregion
}