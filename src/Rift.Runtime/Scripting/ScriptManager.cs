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
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Scripting;
using Rift.Runtime.Fundamental;

namespace Rift.Runtime.Scripting;

internal interface IScriptManagerInternal : IScriptManager, IInitializable
{
    ScriptContext? ScriptContext { get; }
}

internal class ScriptManager : IScriptManagerInternal
{
    private enum Status
    {
        Unknown,
        Init,
        Ready,
        Shutdown
    }

    public ScriptManager(InterfaceBridge bridge)
    {
        _bridge       = bridge;
        Instance      = this;
        ScriptContext = null;
    }

    internal static  ScriptManager   Instance { get; set; } = null!;
    private readonly InterfaceBridge _bridge;

    public  ScriptContext? ScriptContext { get; private set; }
    private Status         _status       { get; set; } = Status.Unknown;

    /// <summary>
    /// Check <seealso cref="ScriptOptions"/> for more details. 
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
    /// Check .csproj file for the list of pre-imported namespaces
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

    private readonly List<string> _importLibraries = [];
    private readonly List<string> _importNamespaces = [];

    public bool Init()
    {
        _status = Status.Init;
        AddLibrary(["Rift.Runtime"]);
        AddNamespace(["Rift.Runtime.Scripting"]);
        _status = Status.Ready;
        return true;
    }

    public void Shutdown()
    {
        RemoveNamespace(["Rift.Runtime.Scripting"]);
        RemoveLibrary(["Rift.Runtime"]);

        _status = Status.Shutdown;
    }

    public void AddLibrary(string library)
    {
        CheckAvailable();

        _importLibraries.Add(library);
    }

    // 估计这里还要做额外处理。。因为还涉及到版本号的问题。
    // 先不管他，之后再说
    public void AddLibrary(IEnumerable<string> libraries)
    {
        CheckAvailable();

        _importLibraries.AddRange(libraries);
    }

    public void RemoveLibrary(string library)
    {
        CheckAvailable();

        _importLibraries.Remove(library);
    }

    public void RemoveLibrary(IEnumerable<string> libraries)
    {
        CheckAvailable();

        foreach (var library in libraries)
        {
            _importLibraries.Remove(library);
        }
    }

    public void AddNamespace(string @namespace)
    {
        CheckAvailable();

        _importNamespaces.Add(@namespace);
    }

    public void AddNamespace(IEnumerable<string> namespaces)
    {
        CheckAvailable();

        _importNamespaces.AddRange(namespaces);
    }

    public void RemoveNamespace(string @namespace)
    {
        CheckAvailable();

        _importNamespaces.Remove(@namespace);
    }

    public void RemoveNamespace(IEnumerable<string> namespaces)
    {
        CheckAvailable();

        foreach (var @namespace in namespaces)
        {
            _importNamespaces.Remove(@namespace);
        }
    }

    public void EvaluateScript(string scriptPath, int timedOutUnitSec = 15)
    {
        CheckAvailable();

        // make sure script context path passed in is canonicalized.
        ScriptContext = new ScriptContext(_bridge, Path.GetFullPath(scriptPath));

        using var loader = new InteractiveAssemblyLoader();
        var loadedAssemblies = CreateLoadedAssembliesMap();

        var runtimeSharedLibraries = new List<Assembly>();

        _importLibraries.ForEach(fileName =>
        {
            var lib = loadedAssemblies.GetValueOrDefault(fileName);
            if (lib is null)
            {
                return;
            }
            runtimeSharedLibraries.Add(lib);
        });

        runtimeSharedLibraries.ForEach(loader.RegisterDependency);

        // 脚本只应该考虑本地文件，不应该考虑跨包引用的情况，哪怕这些包在同一个workspace下。
        // 如果你有这个情况，你更应该思考项目组织是否合理。
        // 如果你有共同引用，你应当根据这个项目写一个插件（写插件的成本几乎为0）
        var resolver = new SourceFileResolver([], ScriptContext.Location);
        var opts = ScriptOptions.Default
            // 相当于你不用额外写using 某个具体的包
            // 比如说你不用额外写using System;
            .AddImports(_preImportedSdkNamespaces)
            .AddImports(_importNamespaces)
            // 运行脚本需要加载的包。
            .AddReferences(_preImportedSdkLibraries)
            .AddReferences(runtimeSharedLibraries)
            .WithSourceResolver(resolver)
            .WithLanguageVersion(LanguageVersion.Default)
            .WithOptimizationLevel(OptimizationLevel.Release);
        var script = CSharpScript.Create(ScriptContext.Text, opts, assemblyLoader: loader);
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
        return AppDomain.CurrentDomain.GetAssemblies().Distinct().GroupBy(a => a.GetName().Name, a => a)
            .Select(gr => new { Name = gr.Key, ResolvedRuntimeAssembly = gr.OrderBy(a => a.GetName().Version).Last() })
            .ToDictionary(f => f.Name ?? throw new InvalidOperationException("Why your assembly is empty?"), f => f.ResolvedRuntimeAssembly, StringComparer.OrdinalIgnoreCase);
    }
}