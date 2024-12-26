using Rift.Runtime.Workspace.Fundamental;

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

    /// <summary>
    /// 是否导出该包的go.mod文件
    /// </summary>
    /// <param name="self"></param>
    /// <param name="export"></param>
    /// <returns></returns>
    public static PackageConfiguration ExportGoMod(this PackageConfiguration self, bool export)
    {
        self.Attributes["Go.Mod.ExportGoMod"] = export;
        return self;
    }


    public static bool ShouldExportGoMod(this PackageConfiguration self)
    {
        return self.Attributes.GetValueOrDefault("Go.Mod.ExportGoMod") as bool? ?? false;
    }
}