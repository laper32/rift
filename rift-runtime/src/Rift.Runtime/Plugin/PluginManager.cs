// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using System.Text.Json;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.API.Workspace;
using Rift.Runtime.Workspace;

namespace Rift.Runtime.Plugin;

internal interface IPluginManagerInternal : IPluginManager, IInitializable
{
    void LoadPlugins();

    bool AddDependencyForPlugin(IPackageImportDeclarator declarator);
    bool AddDependencyForPlugin(IEnumerable<IPackageImportDeclarator> declarators);
    bool AddMetadataForPlugin(string key, object value);
}

internal class PluginManager : IPluginManagerInternal
{
    private readonly PluginIdentities _identities = new();

    private const string LibPathName = "lib";
    private const string PluginEntryToken = "deps.json";
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

    /// <summary>
    /// N.B. 插件系统这里和很多地方不一样的是：我们需要支持没有dll的情况（即：这个插件只有二进制文件，或者只有配置文件）<br/>
    /// 所以必须有一个中间层给插件做Identity。
    /// </summary>
    public void LoadPlugins()
    {
        var workspaceManager = (IWorkspaceManagerInternal)IWorkspaceManager.Instance;
        var declarators = workspaceManager.CollectPluginsForLoad();
        foreach (var declarator in declarators)
        {
            _identities.Add(declarator);
        }

        //_identities.Dump();
        var pendingLoadPlugins = _identities.GetSortedIdentities();
        pendingLoadPlugins.ForEach(x =>
        {
            var libPath = Path.Combine(x.Location, LibPathName);

            // TODO: 大小写敏感的路径支持
            var entryDll = GetEntryDll(x.Value.Name, libPath);

            Console.WriteLine($"Entry dll: {entryDll}");
        });
    }

    private IEnumerable<string> GetPluginSharedAssembliesPath(string libraryPath)
    {
        var dlls = Directory.GetFiles(libraryPath, "*.dll");

        // 为了确保插件之间的接口共享, 我们必须得想一个办法让二进制文件在插件内部共享.
        // 我们不能把每个插件的所有依赖全部共享, 如NuGet里面的东西, 因为确实存在一种情况, 不同插件需要不同的nuget包版本
        // 同时这两个版本互相不兼容, 且这时候两个插件互相独立, 互不干扰.
        // 因此, 我们选择打个洞: 如果你插件需要对外的接口共享, 那么你必须给项目打上assembly级别的标签, 也就是`PluginShared`
        // 然后我们通过读PE的方式找到这些带了这个标签的二进制文件, 把他们收集起来, 再根据一定的规则把他们都变成共享context。
        foreach (var dll in dlls)
        {
            using var fs = new FileStream(dll, FileMode.Open);
            using var pe = new PEReader(fs);
            var reader = pe.GetMetadataReader();
            if (!reader.IsAssembly)
            {
                continue;
            }
            // 首先找[assembly:]那一堆attributes.
            var asmDef = reader.GetAssemblyDefinition();
            // ReSharper disable once ForeachCanBeConvertedToQueryUsingAnotherGetEnumerator
            // ReSharper disable once ForeachCanBePartlyConvertedToQueryUsingAnotherGetEnumerator
            foreach (var attribute in asmDef.GetCustomAttributes())
            {
                var attr = reader.GetCustomAttribute(attribute);
                if (attr.Constructor.Kind != HandleKind.MemberReference)
                {
                    continue;
                }

                var memberReference = reader.GetMemberReference((MemberReferenceHandle)attr.Constructor);
                if (memberReference.Parent.Kind != HandleKind.TypeReference)
                {
                    continue;
                }

                var typeReference = reader.GetTypeReference((TypeReferenceHandle)memberReference.Parent);
                if ($"{reader.GetString(typeReference.Namespace)}.{reader.GetString(typeReference.Name)}".Equals(typeof(PluginShared).FullName!))
                {
                    yield return dll;
                }
            }
        }

        /*

           foreach (var dll in dlls)
           {
               using var fs = new FileStream(dll, FileMode.Open);
               using var reader = new PEReader(fs);
               var metadataReader = reader.GetMetadataReader();
               if (!metadataReader.IsAssembly)
               {
                   continue;
               }
               // 首先找[assembly:]那一堆attributes.
               var assemblyDefinition = metadataReader.GetAssemblyDefinition();
               // ReSharper disable once ForeachCanBeConvertedToQueryUsingAnotherGetEnumerator
               // ReSharper disable once ForeachCanBePartlyConvertedToQueryUsingAnotherGetEnumerator
               foreach (var attribute in assemblyDefinition.GetCustomAttributes())
               {
                   var attr = metadataReader.GetCustomAttribute(attribute);
                   if (attr.Constructor.Kind != HandleKind.MemberReference)
                   {
                       continue;
                   }
           
                   var memberReference = metadataReader.GetMemberReference((MemberReferenceHandle)attr.Constructor);
                   if (memberReference.Parent.Kind != HandleKind.TypeReference)
                   {
                       continue;
                   }
           
                   var typeReference = metadataReader.GetTypeReference((TypeReferenceHandle)memberReference.Parent);
                   if ($"{metadataReader.GetString(typeReference.Namespace)}.{metadataReader.GetString(typeReference.Name)}"
                       == typeof(PluginShared).FullName!)
                   {
                       yield return dll;
                   }
               }
           }*/
    }

    /// <summary>
    /// 获取插件入口dll <br/>
    ///     <remarks>
    ///         需要特别处理文件的大小写问题，这个函数不负责这个！
    ///     </remarks>
    /// </summary>
    /// <param name="pluginName">插件名</param>
    /// <param name="libraryPath">存放插件入口的文件夹路径</param>
    /// <returns></returns>
    private static string GetEntryDll(string pluginName, string libraryPath)
    {
        var conf = Directory.GetFiles(libraryPath, $"*.{PluginEntryToken}");

        // ReSharper disable CommentTypo
        var entryDll = conf.Length != 1                      // 首先看.deps.json的数量
            ? Path.Combine(libraryPath, $"{pluginName}.dll") // 如果没有, 看和文件夹同名的.dll
            : conf[0].Replace($".{PluginEntryToken}", ".dll");         // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
        // ReSharper restore CommentTypo
        return File.Exists(entryDll) ? entryDll : string.Empty;
    }

    public bool AddDependencyForPlugin(IPackageImportDeclarator declarator)
    {
        return _identities.AddDependencyForPlugin(declarator);
    }

    public bool AddDependencyForPlugin(IEnumerable<IPackageImportDeclarator> declarators)
    {
        return _identities.AddDependencyForPlugin(declarators);
    }

    public bool AddMetadataForPlugin(string key, object value)
    {
        return _identities.AddMetadataForPlugin(key, value);
    }
}