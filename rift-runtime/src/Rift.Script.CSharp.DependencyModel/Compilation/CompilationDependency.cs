namespace Rift.Script.CSharp.DependencyModel.Compilation;

public class CompilationDependency(
    string                name,
    string                version,
    IReadOnlyList<string> assemblyPaths,
    IReadOnlyList<string> scriptPaths)
{
    public string Name { get; } = name;

    public string Version { get; } = version;

    public IReadOnlyList<string> AssemblyPaths { get; } = assemblyPaths;

    public IReadOnlyList<string> ScriptPaths { get; } = scriptPaths;

    public override string ToString()
    {
        return $"Name: {Name} , Version: {Version}";
    }
}