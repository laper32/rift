namespace Rift.Script.CSharp.DependencyModel.Compilation;

public class CompilationReference(string path)
{
    public string Path { get; init; } = path;
}