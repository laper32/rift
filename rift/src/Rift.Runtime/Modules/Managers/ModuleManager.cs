using System.Diagnostics;
using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using Rift.Runtime.Application;
using Rift.Runtime.Modules.Attributes;
using Rift.Runtime.Modules.Fundamental;
using Rift.Runtime.Modules.Loader;

namespace Rift.Runtime.Modules.Managers;

public sealed class ModuleManager
{
    private const string ModulePattern = "Rift.Module.*.dll";

    private static   ModuleManager                  _instance            = null!;
    private readonly List<ModuleInstance>           _kernelInstances     = [];
    private readonly List<ModuleLoadContext>        _moduleContexts      = [];
    private readonly List<ModuleIdentity>           _pendingLoadModules  = [];
    private readonly List<ModuleInstance>           _runtimeInstances    = [];
    private readonly List<ModuleSharedAssemblyInfo> _sharedAssemblyInfos = [];

    private readonly ModuleAssemblyContext _sharedContext = new();

    public ModuleManager()
    {
        _instance = this;
    }

    internal static event DelegateModuleUnload? ModuleUnload;

    internal static bool Init()
    {
        _instance.ActivateKernelModules();
        return true;
    }

    internal static void Shutdown()
    {
        _instance.UnloadModules();
    }

    private void UnloadModules()
    {
        UnloadRuntimeModules();
        UnloadKernelModules();

        _sharedContext.Unload();

        foreach (var context in _moduleContexts)
        {
            context.Unload();
        }

        _moduleContexts.Clear();
    }

    private void ActivateRuntimeModules()
    {
        BootRuntimeModules();
        LoadRuntimeModules();
    }

    private void BootRuntimeModules()
    {
    }

    private void LoadRuntimeModules()
    {
        _runtimeInstances.ForEach(x =>
        {
            if (x.Init())
            {
                x.Load();
            }
        });

        _runtimeInstances.ForEach(x => { x.PostLoad(); });
    }

    private void UnloadRuntimeModules()
    {
        _runtimeInstances.ForEach(x =>
        {
            ModuleUnload?.Invoke(x);
            x.Unload(true);
        });

        _runtimeInstances.Clear();
    }

    /// <summary>
    ///     启用内核模块 <br />
    ///     内核模块一定和exe放一起的.
    /// </summary>
    private void ActivateKernelModules()
    {
        BootKernelModules();
        LoadKernelModules();
    }

    private void BootKernelModules()
    {
        LoadKernelModuleIdentities(ApplicationHost.InstallationInformation.LibPath);
        AddKernelSharedAssemblies(ApplicationHost.InstallationInformation.LibPath);

        LoadSharedContext();
        LoadModuleAssemblyContext();
        InitInstances();

        CleanupTemporaries();
    }

    private void LoadKernelModules()
    {
        _kernelInstances.ForEach(x =>
        {
            if (x.Init())
            {
                x.Load();
            }
        });

        _kernelInstances.ForEach(x => { x.PostLoad(); });
    }

    private void UnloadKernelModules()
    {
        _kernelInstances.ForEach(x =>
        {
            ModuleUnload?.Invoke(x);
            x.Unload(true);
        });

        _kernelInstances.Clear();
    }

    private void LoadKernelModuleIdentities(string binPath)
    {
        foreach (var file in Directory.GetFiles(binPath, ModulePattern))
        {
            _pendingLoadModules.Add(new ModuleIdentity(file));
        }
    }

    private void AddKernelSharedAssemblies(string binPath)
    {
        var sharedAssemblies = new Dictionary<
            string,                        // 需要共享的asm文件名
            List<ModuleSharedAssemblyInfo> // 整个插件系统中出现的二进制文件信息
        >();
        var sharedAsmPaths = GetSharedAssemblyPaths(binPath, ModulePattern).ToList();

        foreach (var sharedAsmPath in sharedAsmPaths)
        {
            var name = Path.GetFileNameWithoutExtension(sharedAsmPath);
            if (!sharedAssemblies.TryGetValue(name, out var value))
            {
                value                  = [];
                sharedAssemblies[name] = value;
            }

            value.Add(new ModuleSharedAssemblyInfo(
                sharedAsmPath,
                FileVersionInfo.GetVersionInfo(sharedAsmPath),
                File.GetLastWriteTime(sharedAsmPath))
            );
        }

        // 内核扩展就完全不需要考虑任何加载优先级问题，只要有共享就全部无条件加载。

        // TODO: 之后应该会内核扩展走NuGet的需求，但现在这个版本不考虑。
        // 以后弄
        foreach (var (_, value) in sharedAssemblies)
        {
            _sharedAssemblyInfos.Add(value.First());
        }
    }

    private void LoadModuleAssemblyContext()
    {
        foreach (var identity in _pendingLoadModules)
        {
            _moduleContexts.Add(new ModuleLoadContext(identity, _sharedContext));
        }
    }

    private void LoadSharedContext()
    {
        foreach (var info in _sharedAssemblyInfos)
        {
            _sharedContext.LoadFromAssemblyPath(info.Path);
        }
    }

    private void InitInstances()
    {
        foreach (var context in _moduleContexts)
        {
            _kernelInstances.Add(new ModuleInstance(context));
        }
    }

    private void CleanupTemporaries()
    {
        _sharedAssemblyInfos.Clear();

        _pendingLoadModules.Clear();
    }

    internal static IEnumerable<string> GetSharedAssemblyPaths(string libPath, string searchPatterns)
    {
        var dlls = Directory.GetFiles(libPath, searchPatterns);

        // 为了确保插件之间的接口共享, 我们必须得想一个办法让二进制文件在插件内部共享.
        // 我们不能把每个插件的所有依赖全部共享, 如NuGet里面的东西, 因为确实存在一种情况, 不同插件需要不同的nuget包版本
        // 同时这两个版本互相不兼容, 且这时候两个插件互相独立, 互不干扰.
        // 因此, 我们选择打个洞: 如果你插件需要对外的接口共享, 那么你必须给项目打上assembly级别的标签, 也就是`[ModuleShared]`
        // 然后我们通过读PE的方式找到这些带了这个标签的二进制文件, 把他们收集起来, 再根据一定的规则把他们都变成共享context。
        foreach (var dll in dlls)
        {
            using var fs     = new FileStream(dll, FileMode.Open);
            using var pe     = new PEReader(fs);
            var       reader = pe.GetMetadataReader();
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

                var memberReference = reader.GetMemberReference((MemberReferenceHandle) attr.Constructor);
                if (memberReference.Parent.Kind != HandleKind.TypeReference)
                {
                    continue;
                }


                var typeReference = reader.GetTypeReference((TypeReferenceHandle) memberReference.Parent);
                if ($"{reader.GetString(typeReference.Namespace)}.{reader.GetString(typeReference.Name)}".Equals(
                        typeof(ModuleSharedAttribute).FullName!))
                {
                    yield return dll;
                }
            }
        }
    }

    /// <summary>
    ///     获取插件入口dll <br />
    ///     <remarks>
    ///         - 需要特别处理文件的大小写问题，这个函数不负责这个！ <br />
    ///         - 内核模块不调用这个函数！
    ///     </remarks>
    /// </summary>
    /// <returns> </returns>
    internal static string GetEntryDll(string libPath)
    {
        var    conf = Directory.GetFiles(libPath, "*.deps.json");
        string entryDll;
        // ReSharper disable once ConvertIfStatementToConditionalTernaryExpression
        if (conf.Length != 1) // 首先看.deps.json的数量
        {
            entryDll = Path.Combine(libPath, $"{libPath}.dll"); // 如果没有, 看和文件夹同名的.dll
        }
        else
        {
            entryDll = conf[0].Replace(".deps.json", ".dll"); // 如果超过了1个, 也就是2个或以上, 只看第一个.deps.json及其配套.dll
        }

        return File.Exists(entryDll) ? entryDll : string.Empty;
    }

    internal delegate void DelegateModuleUnload(ModuleInstance instance);
}