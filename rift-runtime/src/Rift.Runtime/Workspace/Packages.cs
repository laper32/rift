using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.Manifest;

namespace Rift.Runtime.Workspace;

internal class Packages
{
    internal readonly Dictionary<string, IMaybePackage> Value = [];

    public void Load(string manifestPath)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EManifestType.Real:
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
                        if (projectManifest.Value.Value.Target is not null)
                        {
                            var targetManifest = projectManifest.Value.Value.Target;
                            var targetPackage = new Package(new Manifest<TargetManifest>(targetManifest), manifestPath);
                            Value.Add(targetPackage.Name, new MaybePackage<Package>(targetPackage));
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

                        var package = new Package(targetManifest.Value, manifestPath);
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
            case EManifestType.Virtual:
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
            default:
            {
                throw new InvalidOperationException("Why are you here?");
            }
        }
    }

    public void LoadRecursively(string manifestPath)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EManifestType.Real:
            {
                switch (manifest)
                {
                    case EitherManifest<Manifest<ProjectManifest>> projectManifest:
                    {
                        if (Value.ContainsKey(projectManifest.Name))
                        {
                            return;
                        }

                        Load(manifestPath);
                        if (projectManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member => Path.Combine(Path.GetDirectoryName(manifestPath)!, member, Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath);
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

                        Load(manifestPath);
                        break;
                    }
                    default:
                    {
                        throw new InvalidOperationException("Why you at here?");
                    }
                }
                break;
            }

            case EManifestType.Virtual:
            {
                switch (manifest)
                {
                    case EitherManifest<VirtualManifest<FolderManifest>> folderManifest:
                    {
                        if (Value.ContainsKey(folderManifest.Name))
                        {
                            return;
                        }

                        Load(manifestPath);

                        if (folderManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member => Path.Combine(Path.GetDirectoryName(manifestPath)!, member, Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath);
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

                        Load(manifestPath);

                        if (workspaceManifest.Value.Value.Members is { } members)
                        {
                            foreach (var fullPath in members.Select(member => Path.Combine(Path.GetDirectoryName(manifestPath)!, member, Definitions.ManifestIdentifier)))
                            {
                                LoadRecursively(fullPath);
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
            default:
            {
                throw new InvalidOperationException("Why you at here?");
            }
        }
    }
}