using Rift.Runtime.Workspace.Fundamental;

namespace Rift.Go.Scripting;

public static class PackageReferenceExtensions
{
    public static PackageReference RequiresGolangVersion(this PackageReference self, string version)
    {
        self.Attributes["GolangVersion"] = version;
        return self;
    }
}