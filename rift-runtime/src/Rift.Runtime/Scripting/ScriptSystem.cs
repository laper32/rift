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

namespace Rift.Runtime.Scripting;

internal interface IScriptSystemInternal : IScriptSystem, IInitializable
{
    public ScriptContext? ScriptContext { get; }
}

internal class ScriptSystem : IScriptSystemInternal
{
    public ScriptSystem()
    {
        IScriptSystem.Instance = this;
        ScriptContext = null;
    }

    public ScriptContext? ScriptContext { get; private set; }

    private bool _init;
    private bool _shutdown;

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
        AddLibraries(["Rift.Runtime"]);
        AddNamespaces(["Rift.Runtime.Scripting"]);
        _init = true;
        _shutdown = false;
        return true;
    }

    public void Shutdown()
    {
        _shutdown = true;
        _init = false;
    }

    // 估计这里还要做额外处理。。因为还涉及到版本号的问题。
    // 先不管他，之后再说
    public void AddLibraries(IEnumerable<string> libraries)
    {
        _importLibraries.AddRange(libraries);
    }

    public void AddNamespaces(IEnumerable<string> namespaces)
    {
        _importNamespaces.AddRange(namespaces);
    }

    public void EvaluateScript(string scriptPath, int timedOutUnitSec = 15)
    {
        // make sure script context path passed in is canonicalized.
        ScriptContext = new ScriptContext(Path.GetFullPath(scriptPath));

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

    private static Dictionary<string, Assembly> CreateLoadedAssembliesMap()
    {
        // Build up a map of loaded assemblies that picks runtime assembly with the highest version.
        // This aligns with the CoreCLR that uses the highest version strategy.
        return AppDomain.CurrentDomain.GetAssemblies().Distinct().GroupBy(a => a.GetName().Name, a => a)
            .Select(gr => new { Name = gr.Key, ResolvedRuntimeAssembly = gr.OrderBy(a => a.GetName().Version).Last() })
            .ToDictionary(f => f.Name ?? throw new InvalidOperationException("Why your assembly is empty?"), f => f.ResolvedRuntimeAssembly, StringComparer.OrdinalIgnoreCase);
    }
}