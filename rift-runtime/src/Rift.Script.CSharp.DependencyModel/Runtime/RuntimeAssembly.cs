using System.Reflection;

namespace Rift.Script.CSharp.DependencyModel.Runtime;

public class RuntimeAssembly(AssemblyName name, string path)
{
    public AssemblyName Name { get; } = name;
    public string       Path { get; } = path;

    public override int GetHashCode()
    {
        return Name.GetHashCode() ^ Path.GetHashCode();
    }

    public override bool Equals(object? obj)
    {
        if (obj is not RuntimeAssembly other)
        {
            return false;
        }

        return other.Name == Name && other.Path == Path;
    }
        
    public override string ToString()
    {
        return $"{nameof(Name)}: {Name}, {nameof(Path)}: {Path}";
    }
}