using System.Collections.ObjectModel;
using Microsoft.CodeAnalysis.Text;

namespace Rift.Script.CSharp.Fundamental;

public interface IScriptContext
{
    public SourceText Code { get; }

    public string WorkingDirectory { get; }

    public IReadOnlyList<string> Args { get; }

    public string FilePath { get; }
    
    public IReadOnlyList<string> PackageSources { get; }
}

public class ScriptContext(
    SourceText code,
    string workingDirectory,
    IEnumerable<string> args,
    string filePath,
    IEnumerable<string> packageSources) : IScriptContext
{
    public SourceText Code { get; init; } = code;
    public string WorkingDirectory { get; init; } = workingDirectory;
    public IReadOnlyList<string> Args { get; init; } = new ReadOnlyCollection<string>(args.ToArray());
    public string FilePath { get; init; } = filePath;

    public IReadOnlyList<string> PackageSources { get; init; } =
        new ReadOnlyCollection<string>(packageSources.ToArray());

}