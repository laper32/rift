// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Diagnostics;
using System.Text.Json;
using Rift.Runtime.Constants;
using Rift.Runtime.IO;
using Rift.Runtime.Manifest;
using Rift.Runtime.Manifest.Real;
using Rift.Runtime.Manifest.Virtual;
using Rift.Runtime.Workspace.Graph;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Workspace.Fundamental;

internal class Package(IManifest manifest, string manifestPath)
{
    public string    Name         => manifest.Name;
    public string    Version      => manifest.Version;
    public string    ManifestPath => manifestPath;
    public string    Root         => Directory.GetParent(ManifestPath)!.FullName;
    public IManifest Value        => manifest;

    public string? Dependencies
    {
        get
        {
            if (manifest.Dependencies is { } dependencies)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies));
            }

            return null;
        }
    }

    public string? Plugins
    {
        get
        {
            if (manifest.Plugins is { } plugins)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, plugins));
            }

            return null;
        }
    }

    public string? Configure
    {
        get
        {
            if (manifest.Configure is { } configure)
            {
                return Path.GetFullPath(WorkspaceManager.GetActualScriptPath(ManifestPath, configure));
            }

            return null;
        }
    }

    public Dictionary<string, JsonElement> Others => manifest.Others;
}

internal class Packages
{
    internal readonly Dictionary<string, IMaybePackage> Value = [];

    public void Load(string manifestPath, IEitherManifest? parentPackage = null)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EEitherManifest.Real:
            {
                switch (manifest)
                {
                    case EitherManifest<Manifest<ProjectManifest>> projectManifest:
                    {
                        if (Value.ContainsKey(projectManifest.Name))
                        {
                            throw new InvalidOperationException($"Package already exists: `{projectManifest.Name}`");
                        }

                        var package = new Package(projectManifest.Value, manifestPath);

                        var packageNode = new PackageGraphNode(package.Name, package.Version);

                        if (parentPackage is null)
                        {
                            packageNode.MarkAsRoot();
                        }

                        WorkspaceManager.PackageGraph.Add(packageNode);

                        if (projectManifest.Value.Value.Target is not null)
                        {
                            var targetManifest = projectManifest.Value.Value.Target;
                            var targetPackage = new Package(new Manifest<TargetManifest>(targetManifest), manifestPath);

                            var targetPackageNode = new PackageGraphNode(targetPackage.Name, targetPackage.Version);

                            WorkspaceManager.PackageGraph.Add(targetPackageNode);
                            WorkspaceManager.PackageGraph.Connect(targetPackageNode, packageNode);

                                    Value.Add(targetPackage.Name, new MaybePackage<Package>(targetPackage));
                        }

                        if (parentPackage is not null)
                        {
                            if (WorkspaceManager.PackageGraph.Find(parentPackage.Name, parentPackage.Version) is not
                                { } parent)
                            {
                                var node = new PackageGraphNode(parentPackage.Name, parentPackage.Version);
                                WorkspaceManager.PackageGraph.Add(node);
                                WorkspaceManager.PackageGraph.Connect(packageNode, node);
                            }
                            else
                            {
                                WorkspaceManager.PackageGraph.Connect(packageNode, parent);
                            }
                        }

                        Value.Add(package.Name, new MaybePackage<Package>(package));
                        break;
                    }
                    case EitherManifest<Manifest<TargetManifest>> targetManifest:
                    {
                        if (Value.ContainsKey(targetManifest.Name))
                        {
                            throw new InvalidOperationException($"Package already exists: `{targetManifest.Name}`");
                        }

                        var package     = new Package(targetManifest.Value, manifestPath);
                        var packageNode = new PackageGraphNode(package.Name, package.Version);

                        if (parentPackage is null)
                        {
                            packageNode.MarkAsRoot();
                        }

                        WorkspaceManager.PackageGraph.Add(packageNode);

                        if (parentPackage is not null)
                        {
                            if (WorkspaceManager.PackageGraph.Find(parentPackage.Name, parentPackage.Version) is not
                                { } parent)
                            {
                                var node = new PackageGraphNode(parentPackage.Name, parentPackage.Version);
                                WorkspaceManager.PackageGraph.Add(node);
                                WorkspaceManager.PackageGraph.Connect(packageNode, node);
                            }
                            else
                            {
                                WorkspaceManager.PackageGraph.Connect(packageNode, parent);
                            }
                        }

                        Value.Add(package.Name, new MaybePackage<Package>(package));
                        break;
                    }
                    default:
                    {
                        throw new InvalidOperationException("Why you at here?");
                    }
                }

                break;
            }
            case EEitherManifest.Virtual:
            {
                switch (manifest)
                {
                    case EitherManifest<VirtualManifest<FolderManifest>> folderManifest:
                    {
                        if (Value.ContainsKey(folderManifest.Name))
                        {
                            throw new InvalidOperationException($"Package already exists: `{folderManifest.Name}`");
                        }

                        var virtualPackage = new VirtualPackage(folderManifest.Value, manifestPath);

                        var packageNode    = new PackageGraphNode(virtualPackage.Name, virtualPackage.Version);

                        if (parentPackage is null)
                        {
                            packageNode.MarkAsRoot();
                        }

                        WorkspaceManager.PackageGraph.Add(packageNode);

                        if (parentPackage is not null)
                        {
                            if (WorkspaceManager.PackageGraph.Find(parentPackage.Name, parentPackage.Version) is not
                                { } parent)
                            {
                                var node = new PackageGraphNode(parentPackage.Name, parentPackage.Version);
                                WorkspaceManager.PackageGraph.Add(node);
                                WorkspaceManager.PackageGraph.Connect(packageNode, node);
                            }
                            else
                            {
                                WorkspaceManager.PackageGraph.Connect(packageNode, parent);
                            }
                        }

                        Value.Add(virtualPackage.Name, new MaybePackage<VirtualPackage>(virtualPackage));
                        break;
                    }
                    case EitherManifest<VirtualManifest<WorkspaceManifest>> workspaceManifest:
                    {
                        if (Value.ContainsKey(workspaceManifest.Name))
                        {
                            throw new InvalidOperationException($"Package already exists: `{workspaceManifest.Name}`");
                        }

                        var virtualPackage = new VirtualPackage(workspaceManifest.Value, manifestPath);
                        var packageNode    = new PackageGraphNode(virtualPackage.Name, virtualPackage.Version);

                        if (parentPackage is null)
                        {
                            packageNode.MarkAsRoot();
                        }

                        WorkspaceManager.PackageGraph.Add(packageNode);

                        if (parentPackage is not null)
                        {
                            if (WorkspaceManager.PackageGraph.Find(parentPackage.Name, parentPackage.Version) is not
                                { } parent)
                            {
                                var node = new PackageGraphNode(parentPackage.Name, parentPackage.Version);
                                WorkspaceManager.PackageGraph.Add(node);
                                WorkspaceManager.PackageGraph.Connect(packageNode, node);
                            }
                            else
                            {
                                WorkspaceManager.PackageGraph.Connect(packageNode, parent);
                            }
                        }

                        Value.Add(virtualPackage.Name, new MaybePackage<VirtualPackage>(virtualPackage));
                        break;
                    }
                    default:
                    {
                        throw new InvalidOperationException("Why you at here?");
                    }
                }

                break;
            }
            case EEitherManifest.Rift:
            {
                throw new InvalidOperationException("Unable to parse Rift-specific manifests.");
            }
            default:
            {
                throw new UnreachableException();
            }
        }
    }

    public void LoadRecursively(string manifestPath, IEitherManifest? parentManifest = null)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EEitherManifest.Real:
            {
                switch (manifest)
                {
                    case EitherManifest<Manifest<ProjectManifest>> projectManifest:
                    {
                        if (Value.ContainsKey(projectManifest.Name))
                        {
                            return;
                        }

                        Load(manifestPath, parentManifest);
                        if (projectManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member =>
                                         Path.Combine(Path.GetDirectoryName(manifestPath)!, member,
                                             Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath, projectManifest);
                            }
                        }

                        break;
                    }
                    case EitherManifest<Manifest<TargetManifest>> targetManifest:
                    {
                        if (Value.ContainsKey(targetManifest.Name))
                        {
                            return;
                        }

                        // 这个时候如果ParentManifest不是空的话，那么parent应当有以下之一：`[workspace]`, `[folder]`, `[project]`
                        Load(manifestPath, parentManifest);
                        break;
                    }
                    default:
                    {
                        throw new InvalidOperationException("Why you at here?");
                    }
                }

                break;
            }

            case EEitherManifest.Virtual:
            {
                switch (manifest)
                {
                    case EitherManifest<VirtualManifest<FolderManifest>> folderManifest:
                    {
                        if (Value.ContainsKey(folderManifest.Name))
                        {
                            return;
                        }

                        Load(manifestPath, parentManifest);

                        if (folderManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member =>
                                         Path.Combine(Path.GetDirectoryName(manifestPath)!, member,
                                             Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath, folderManifest);
                            }
                        }

                        break;
                    }

                    case EitherManifest<VirtualManifest<WorkspaceManifest>> workspaceManifest:
                    {
                        if (Value.ContainsKey(workspaceManifest.Name))
                        {
                            return;
                        }

                        // 这个是绝对不可能还有parent的！
                        Load(manifestPath);

                        if (workspaceManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member =>
                                         Path.Combine(Path.GetDirectoryName(manifestPath)!, member,
                                             Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath, workspaceManifest);
                            }
                        }

                        break;
                    }
                    default:
                    {
                        throw new InvalidOperationException("Why you at here?");
                    }
                }

                break;
            }
            case EEitherManifest.Rift:
            {
                throw new InvalidOperationException("Unable to parse Rift specific manifests.");
            }
            default:
            {
                throw new InvalidOperationException("Why you at here?");
            }
        }
    }

    public void DumpPackagesMetadata()
    {
        Tty.WriteLine("DumpPackagesMetadata...");
        var str = JsonSerializer.Serialize(Value, new JsonSerializerOptions
        {
            WriteIndented = true
        });
        Tty.WriteLine(str);
        Tty.WriteLine("...End");
    }
}