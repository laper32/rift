using Rift.Runtime.Workspace;

namespace Rift.Go.Scripting;

public static class PackageConfigurationExtensions
{
    public static PackageConfiguration GolangVersion(this PackageConfiguration self, string version)
    {
        self.Attributes["Go.Version"] = version;
        return self;
    }

    public static string GetGolangVersion(this PackageConfiguration self)
    {
        return self.Attributes.GetValueOrDefault("Go.Version") as string ?? string.Empty;
    }

    public static PackageConfiguration WriteGoModToPath(this PackageConfiguration self, string path)
    {
        self.Attributes["Go.Mod.Path"] = path;
        return self;
    }

    public static string GetWriteModToPath(this PackageConfiguration self)
    {
        return self.Attributes.GetValueOrDefault("Go.Mod.Path") as string ?? string.Empty;
    }
}