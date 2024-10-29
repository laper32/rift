namespace Rift.Script.CSharp.DependencyModel.Runtime;

public class RuntimeDependency(string name, string version, IReadOnlyList<RuntimeAssembly> assemblies, IReadOnlyList<string> nativeAssets, IReadOnlyList<string> scriptPaths)
{
    public string                         Name         { get; init; } = name;
    public string                         Version      { get; init; } = version;
    public IReadOnlyList<RuntimeAssembly> Assemblies   { get; init; } = assemblies;
    public IReadOnlyList<string>          NativeAssets { get; init; } = nativeAssets;
    public IReadOnlyList<string>          ScriptPaths  { get; init; } = scriptPaths;
}
