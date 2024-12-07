using Rift.Runtime.Workspace;

namespace Rift.Go.Scripting;

public static class PackageConfigurationExtensions
{
    public static PackageConfiguration GolangVersion(this PackageConfiguration self, string version)
    {
        self.Attributes["Go.Version"] = version;
        return self;
    }
}