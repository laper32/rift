// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manager;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.API.Schema;
using Tomlyn;

namespace Rift.Runtime.Manager;

public class Package(IManifest manifest, string manifestPath)
{
    public string Name => manifest.Name;
    public string ManifestPath => manifestPath;
    public string Root => Directory.GetParent(ManifestPath)!.FullName;

    public string? Dependencies
    {
        get
        {
            if (manifest.Dependencies is {} dependencies)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies);
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
                return WorkspaceManager.GetActualScriptPath(ManifestPath, plugins);
            }

            return null;
        }
    }

    public string? Metadata
    {
        get
        {
            if (manifest.Metadata is { } metadata)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, metadata);
            }

            return null;
        }
    }
}

public class VirtualPackage(IVirtualManifest virtualManifest, string manifestPath)
{
    public string Name => virtualManifest.Name;
    public string ManifestPath => manifestPath;
    public string Root => Directory.GetParent(ManifestPath)!.FullName;

    public string? Dependencies
    {
        get
        {
            if (virtualManifest.Dependencies is {} dependencies)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, dependencies);
            }

            return null;
        }
    }

    public string? Plugins
    {
        get
        {
            if (virtualManifest.Plugins is { } plugins)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, plugins);
            }

            return null;
        }
    }

    public string? Metadata
    {
        get
        {
            if (virtualManifest.Metadata is { } metadata)
            {
                return WorkspaceManager.GetActualScriptPath(ManifestPath, metadata);
            }

            return null;
        }
    }
}

internal interface IMaybePackage
{
    public enum EMaybePackage
    {
        Package,
        Virtual
    }

    public string Name { get; }
    public string? Dependencies { get; }
    public string? Plugins { get; }
    public string? Metadata { get; }
}

internal class MaybePackage<T>(T data) : IMaybePackage
{
    public IMaybePackage.EMaybePackage PackageType { get; } = data switch
    {
        Package package => IMaybePackage.EMaybePackage.Package,
        VirtualPackage package => IMaybePackage.EMaybePackage.Virtual,
        _ => throw new InvalidOperationException("Only accepts `Package` or `VirtualPackage`.")
    };

    public string Name => data switch
    {
        Package package => package.Name,
        VirtualPackage package => package.Name,
        _ => string.Empty
    };

    public string? Dependencies => data switch
    {
        Package package => package.Dependencies ,
        VirtualPackage package => package.Dependencies,
        _ => null
    };

    public string? Plugins => data switch
    {
        Package package => package.Plugins,
        VirtualPackage package => package.Plugins,
        _ => null
    };

    public string? Metadata => data switch
    {
        Package package => package.Metadata,
        VirtualPackage package => package.Metadata,
        _ => null
    };
}


internal interface IWorkspaceManagerInternal : IWorkspaceManager, IInitializable;

public class WorkspaceManager : IWorkspaceManagerInternal
{
    internal readonly Dictionary<string, IMaybePackage> Packages = [];

    public WorkspaceManager()
    {
        IWorkspaceManager.Instance = this;
    }

    public bool Init()
    {
        const string filePath = @"D:\\workshop\\projects\\common_usage\\.py\\generic\\rift-workspace\\Rift.toml";
        Console.WriteLine($"{filePath} => IsDir => {Directory.Exists(filePath)}");
        Console.WriteLine($"{filePath} => IsFile => {File.Exists(filePath)}");

        const string dirPath = @"D:\\workshop\\projects\\common_usage\\.py\\generic\\rift-workspace";
        Console.WriteLine($"{dirPath} => IsDir => {Directory.Exists(dirPath)}");
        Console.WriteLine($"{dirPath} =>IsFile => {File.Exists(dirPath)}");
        return true;
    }

    public void Shutdown()
    {
    }

    public void LoadWorkspace(string path)
    {
        // NOTE: 现在只考虑根目录的情况，不考虑从下往上搜的情况（因为从下到上需要带Context。）
        // 现在我们没办法处理这个问题，得先自顶向下正确解析了才能处理自底向上的问题。
        Root = GetRootManifest(path);
    }

    public TomlManifest? LoadManifest(string path)
    {
        try
        {
            var text = File.ReadAllText(path);
            var content = Toml.ToModel<TomlManifest>(text);
            return content;
        }
        catch (Exception)
        {
            return null;
        }
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

    //public IEitherManifest? ReadManifest(string path)
    //{
    //    var deserializedManifest = LoadManifest(path);
    //    if (deserializedManifest is not null)
    //    {
    //        if (deserializedManifest.Workspace is {} workspace)
    //        {
    //            if (deserializedManifest.Folder is not null || deserializedManifest.Project is not null ||
    //                deserializedManifest.Target is not null)
    //            {
    //                throw new InvalidOperationException("Workspace manifest cannot contain project, folder or target.");
    //            }

    //        }


    //        return new EitherManifest();
    //    }
    //    else
    //    {
    //        return null;
    //    }
    //}
    public string Root { get; internal set; }

    //public IEitherManifest<T>? ReadManifest<T>(string path) where T : class
    //{
    //    var deserializedManifest = LoadManifest(path);

    //    if (deserializedManifest is not null)
    //    {
    //        if (deserializedManifest.Workspace is { } workspace)
    //        {
    //            if (deserializedManifest.Folder is not null || deserializedManifest.Project is not null ||
    //                deserializedManifest.Target is not null)
    //            {
    //                throw new InvalidOperationException("Workspace manifest cannot contain project, folder or target.");
    //            }

    //            var workspaceNameActual = "";
    //            var workspaceName = workspace.Name ?? string.Empty;
    //            workspaceName = workspaceName.Trim();
    //            if (string.IsNullOrEmpty(workspaceName))
    //            {
    //                var manifestLocation = Path.GetDirectoryName(path);
    //            }
    //            else
    //            {
                    
    //            }

    //            /*
    //              let workspace_manifest = manifest.workspace.unwrap();
    //                                  // 如果workspace manifest里面没有指定名字，那么我们就认为这个workspace的文件夹名字就是其名字。
    //                                  // 毕竟按理说这里就是root。。。所以这么写也没什么问题。
    //                                  let mut workspace_name = "";
    //                                  if workspace_manifest.name.is_none() {
    //                                      let manifest_location = path.parent().unwrap();
    //                                      let workspace_root = WorkspaceManager::instance().root();
    //                                      if workspace_root.eq(manifest_location) {
    //                                          workspace_name =
    //                                              manifest_location.file_name().unwrap().to_str().unwrap();
    //                                      }
    //                                  } else {
    //                                      workspace_name = workspace_manifest.name.as_ref().unwrap();
    //                                  }
    //             */
                
    //        }
    //    }

    //    return null;
    //}
}