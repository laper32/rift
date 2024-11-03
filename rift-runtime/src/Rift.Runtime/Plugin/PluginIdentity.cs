using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Manifest;
using Rift.Runtime.Manifest;
using Rift.Runtime.Workspace;

// 我感觉逃不了课的，不管怎么操作最后都要回到扫一遍你有没有这个插件包

namespace Rift.Runtime.Plugin;

internal class PluginIdentity(IMaybePackage package)
{
    public IMaybePackage Value { get; init; } = package;
}

internal class PluginIdentities
{
    private const    string PluginDirectoryName     = "plugins";
    private const    string PluginLibraryName       = "lib";       // eg. ~/.rift/plugins/Example/lib/Example.dll 
    private readonly string _installationPluginPath = Path.Combine(IRuntime.Instance.InstallationPath, PluginDirectoryName);
    private readonly string _userPluginPath         = Path.Combine(IRuntime.Instance.UserPath, PluginDirectoryName);

    // TODO: 之后可以参考python怎么处理多源的情况的。
    // 现在我们只会考虑安装路径和用户路径。

    private readonly List<PluginIdentity> _installationPluginIdentities = [];
    private readonly List<PluginIdentity> _userHomePluginIdentities     = [];

    private PluginIdentity CreatePluginIdentity(string manifestPath)
    {
        var manifest = WorkspaceManager.ReadManifest(manifestPath);
        switch (manifest.Type)
        {
            case EManifestType.Rift:
            {
                switch (manifest)
                {
                    // 插件是存在一个文件夹里通过版本号作为文件夹区分方式的，所以不能直接打死！
                    case EitherManifest<RiftManifest<PluginManifest>> pluginManifest:
                    {
                        var riftPackage = new RiftPackage(pluginManifest.Value, manifestPath);
                        return new PluginIdentity(new MaybePackage<RiftPackage>(new RiftPackage(pluginManifest.Value, manifestPath)));
                    }
                    default:
                    {
                        throw new InvalidOperationException("Only supports Rift specific manifests."); 
                    }
                }
            }
            case EManifestType.Virtual:
            case EManifestType.Real:
            default:
            {
                throw new InvalidOperationException("Only supports Rift specific manifests.");
            }
        }
    }

    // 插件系统不可能出现套娃情况的，所有插件只会有一层。

    // 想了想，似乎可以不用非得扫一遍
    public void EnumerateInstallationPathPlugins()
    {
        var pluginsPath = Directory.GetDirectories(_installationPluginPath);
        foreach (var pluginPath in pluginsPath)
        {
            var versionsPath = Directory.GetDirectories(pluginPath);
            foreach (var versionPath in versionsPath)
            {
                Console.WriteLine($"{pluginPath} => {versionPath}");
            }
        }
    }
}