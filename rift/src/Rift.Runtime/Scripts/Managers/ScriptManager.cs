// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Reflection;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;
using Microsoft.CodeAnalysis.Scripting.Hosting;
using Rift.Runtime.Scripts.Fundamental;

namespace Rift.Runtime.Scripts.Managers;

public sealed class ScriptManager
{
    private static ScriptManager _instance = null!;

    private readonly List<string> _libraries  = [];
    private readonly List<string> _namespaces = [];

    /// <summary>
    ///     Check <seealso cref="ScriptOptions" /> for more details.
    /// </summary>
    private readonly IEnumerable<string> _preImportedSdkLibraries =
    [
        "System.Collections",
        "System.Collections.Concurrent",
        "System.Console",
        "System.Diagnostics.Debug",
        "System.Diagnostics.Process",
        "System.Diagnostics.StackTrace",
        "System.Globalization",
        "System.IO",
        "System.IO.FileSystem",
        "System.IO.FileSystem.Primitives",
        "System.Reflection",
        "System.Reflection.Extensions",
        "System.Reflection.Primitives",
        "System.Runtime",
        "System.Runtime.Extensions",
        "System.Runtime.InteropServices",
        "System.Text.Encoding",
        "System.Text.Encoding.CodePages",
        "System.Text.Encoding.Extensions",
        "System.Text.RegularExpressions",
        "System.Threading",
        "System.Threading.Tasks",
        "System.Threading.Tasks.Parallel",
        "System.Threading.Thread",
        "System.ValueTuple"
    ];

    /// <summary>
    ///     Check .csproj file for the list of pre-imported namespaces
    /// </summary>
    private readonly IEnumerable<string> _preImportedSdkNamespaces =
    [
        "System",
        "System.IO",
        "System.Collections.Generic",
        "System.Console",
        "System.Diagnostics",
        "System.Text",
        "System.Threading.Tasks",
        "System.Linq"
    ];
    
    private readonly IEnumerable<string> _runtimeImportNamespaces =
    [
        "Rift.Runtime.Scripts.Scripting"
    ];

    private Status _status = Status.Unknown;


    public ScriptManager()
    {
        _instance     = this;
        ScriptContext = null;
    }

    internal static ScriptContext? ScriptContext { get; private set; }

    internal static bool Init()
    {
        return _instance.InitInternal();
    }

    internal static void Shutdown()
    {
        _instance.ShutdownInternal();
    }

    internal static void EvaluateScript(string scriptPath, int timedOutUnitSec = 15)
    {
        _instance.EvaluateScriptInternal(scriptPath, timedOutUnitSec);
    }

    /// <summary>
    ///     为脚本系统添加依赖库 <br />
    ///     <remarks>
    ///         如果你的插件需要对脚本系统进行扩展，必须在<see cref="IPlugin.OnLoad" /> 或 <see cref="IPlugin.OnAllLoaded" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="library"> </param>
    public static void AddLibrary(string library)
    {
        _instance.AddLibraryInternal([library]);
    }

    /// <summary>
    ///     为脚本系统添加依赖库 <br />
    ///     <remarks>
    ///         如果你的插件需要对脚本系统进行扩展，必须在<see cref="IPlugin.OnLoad" /> 或 <see cref="IPlugin.OnAllLoaded" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="libraries"> </param>
    public static void AddLibrary(IEnumerable<string> libraries)
    {
        _instance.AddLibraryInternal(libraries);
    }

    /// <summary>
    ///     为脚本系统移除依赖库 <br />
    ///     <remarks>
    ///         如果你在插件中使用了<see cref="AddLibrary(string)" /> 或者<see cref="AddLibrary(IEnumerable{string})" />，则你需要在
    ///         <see cref="IPlugin.OnUnload" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="library"> </param>
    public static void RemoveLibrary(string library)
    {
        _instance.RemoveLibraryInternal([library]);
    }

    /// <summary>
    ///     为脚本系统移除依赖库 <br />
    ///     <remarks>
    ///         如果你在插件中使用了<see cref="AddLibrary(string)" /> 或者<see cref="AddLibrary(IEnumerable{string})" />，则你需要在
    ///         <see cref="IPlugin.OnUnload" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="libraries"> </param>
    public static void RemoveLibrary(IEnumerable<string> libraries)
    {
        _instance.RemoveLibraryInternal(libraries);
    }

    /// <summary>
    ///     为脚本系统添加Namespace <br />
    ///     <remarks>
    ///         如果你的插件需要对脚本系统进行扩展，必须在<see cref="IPlugin.OnLoad" /> 或 <see cref="IPlugin.OnAllLoaded" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="namespace"> </param>
    public static void AddNamespace(string @namespace)
    {
        _instance.AddNamespaceInternal([@namespace]);
    }

    /// <summary>
    ///     为脚本系统添加Namespace <br />
    ///     <remarks>
    ///         如果你的插件需要对脚本系统进行扩展，必须在<see cref="IPlugin.OnLoad" /> 或 <see cref="IPlugin.OnAllLoaded" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="namespaces"> </param>
    public static void AddNamespace(IEnumerable<string> namespaces)
    {
        _instance.AddNamespaceInternal(namespaces);
    }

    /// <summary>
    ///     为脚本系统移除Namespace <br />
    ///     <remarks>
    ///         如果你在插件中使用了<see cref="AddNamespace(string)" /> 或者<see cref="AddNamespace(IEnumerable{string})" />，则你需要在
    ///         <see cref="IPlugin.OnUnload" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="namespace"> </param>
    public static void RemoveNamespace(string @namespace)
    {
        _instance.RemoveNamespaceInternal([@namespace]);
    }

    /// <summary>
    ///     为脚本系统移除Namespace <br />
    ///     <remarks>
    ///         如果你在插件中使用了<see cref="AddNamespace(string)" /> 或者<see cref="AddNamespace(IEnumerable{string})" />，则你需要在
    ///         <see cref="IPlugin.OnUnload" />中调用该函数
    ///     </remarks>
    /// </summary>
    /// <param name="namespaces"> </param>
    public static void RemoveNamespace(IEnumerable<string> namespaces)
    {
        _instance.RemoveNamespaceInternal(namespaces);
    }


    private IEnumerable<string> GetRuntimeImportNamespaces() => _runtimeImportNamespaces;

    internal bool InitInternal()
    {
        _status = Status.Init;
        AddLibrary(["Rift.Runtime"]);
        AddNamespace(_instance.GetRuntimeImportNamespaces());
        _status = Status.Ready;
        return true;
    }

    internal void ShutdownInternal()
    {
        RemoveNamespace(_instance.GetRuntimeImportNamespaces());
        RemoveLibrary(["Rift.Runtime"]);
        _status = Status.Shutdown;
    }

    private void AddLibraryInternal(IEnumerable<string> libraries)
    {
        CheckAvailable();

        _libraries.AddRange(libraries);
    }

    private void RemoveLibraryInternal(IEnumerable<string> libraries)
    {
        CheckAvailable();

        foreach (var library in libraries)
        {
            _libraries.Remove(library);
        }
    }

    private void AddNamespaceInternal(IEnumerable<string> namespaces)
    {
        CheckAvailable();
        _namespaces.AddRange(namespaces);
    }

    private void RemoveNamespaceInternal(IEnumerable<string> namespaces)
    {
        CheckAvailable();
        foreach (var @namespace in namespaces)
        {
            _namespaces.Remove(@namespace);
        }
    }

    /// <summary>
    ///     执行脚本
    /// </summary>
    /// <param name="scriptPath"> </param>
    /// <param name="timedOutUnitSec"> </param>
    internal void EvaluateScriptInternal(string scriptPath, int timedOutUnitSec = 15)
    {
        /*
         * 执行脚本的时候有几点需要格外注意：
         * 1. Assembly必须是LoadFromFile，绝对不能LoadFromStream。
         *    Roslyn不支持加载Dynamic Assembly，这会直接导致脚本无法读取Assembly的所在位置，甚至哪怕你加载了也拿不到
         * 想要的类型
         *
         * 参阅:
         *  - https://github.com/dotnet/roslyn/blob/main/docs/wiki/Scripting-API-Samples.md
         *  - https://github.com/dotnet/roslyn/issues/6101
         *
         * 2. 数据共享问题。
         *    目前来看，脚本系统只认Host自身加载的数据，不认动态加载的数据
         * 注：这里所谓的‘动态加载的数据’指的是形如InterfaceManager那边注册的接口。
         *    在导出函数给脚本系统使用时需要非常小心，必须要控制脚本域和插件域，哪怕你想导出API给脚本，如果你的函数实现中
         * 有调用InterfaceManager中的接口，也会直接报数据不存在。
         *
         *    目前我的猜测可能需要类似MonoMod这种IL注入才行，但还需要额外测试。
         */

        CheckAvailable();

        // make sure script context path passed in is canonicalized.
        ScriptContext = new ScriptContext(Path.GetFullPath(scriptPath));

        var loadedAssemblies = CreateLoadedAssembliesMap();

        var runtimeReferences = new List<Assembly>();

        _libraries.ForEach(AddRuntimeReferences);

        // 脚本只应该考虑本地文件，不应该考虑跨包引用的情况，哪怕这些包在同一个workspace下。
        // 如果你有这个情况，你更应该思考项目组织是否合理。
        // 如果你有共同引用，你应当根据这个项目写一个插件（写插件的成本几乎为0）
        var resolver = new SourceFileResolver([], ScriptContext.Location);
        var opts = ScriptOptions.Default
            // 相当于你不用额外写using 某个具体的包
            // 比如说你不用额外写using System;
            .AddImports(_preImportedSdkNamespaces)
            .AddImports(_namespaces)
            // 运行脚本需要加载的包。
            .AddReferences(_preImportedSdkLibraries)
            .AddReferences(runtimeReferences)
            //.AddReferences(pluginReferences)
            .WithSourceResolver(resolver)
            .WithLanguageVersion(LanguageVersion.Default)
            .WithOptimizationLevel(OptimizationLevel.Release);
        var script  = CSharpScript.Create(ScriptContext.Text, opts, assemblyLoader: new InteractiveAssemblyLoader());
        var compile = script.Compile();
        if (compile.Any())
        {
            Console.WriteLine($"Error found when compiling: {scriptPath}");
            foreach (var diagnostic in compile)
            {
                Console.WriteLine(diagnostic.GetMessage());
            }
        }
        else
        {
            script.RunAsync().Wait(TimeSpan.FromSeconds(timedOutUnitSec));

            // reset.
            ScriptContext = null;
        }

        return;

        void AddRuntimeReferences(string fileName)
        {
            var lib = loadedAssemblies.GetValueOrDefault(fileName);
            if (lib is null)
            {
                return;
            }

            if (runtimeReferences.Contains(lib))
            {
                return;
            }

            runtimeReferences.Add(lib);
        }
    }

    private void CheckAvailable()
    {
        if (_status is not (Status.Init or Status.Ready))
        {
            throw new InvalidOperationException("ScriptManager is not available");
        }
    }

    private static Dictionary<string, Assembly> CreateLoadedAssembliesMap()
    {
        // Build up a map of loaded assemblies that picks runtime assembly with the highest version.
        // This aligns with the CoreCLR that uses the highest version strategy.
        return AppDomain
            .CurrentDomain
            .GetAssemblies()
            .Distinct()
            .GroupBy(a => a.GetName().Name, a => a)
            .Select(gr => new
            {
                Name = gr.Key,
                ResolvedRuntimeAssembly = gr
                    .OrderBy(a => a.GetName().Version)
                    .Last()
            })
            .ToDictionary(
                f => f.Name ?? throw new InvalidOperationException("Why your assembly name is empty?"),
                f => f.ResolvedRuntimeAssembly, StringComparer.OrdinalIgnoreCase
            );
    }

    private enum Status
    {
        Unknown,
        Init,
        Ready,
        Shutdown
    }
}