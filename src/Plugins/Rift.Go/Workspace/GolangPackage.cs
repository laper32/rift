using Rift.Runtime.Workspace;

namespace Rift.Go.Workspace;

internal class GolangPackage(IPackageInstance package)
{
    internal IPackageInstance Instance { get; init; } = package;

    public string Name         { get; init; } = package.Name;
    public string ManifestPath { get; init; } = package.ManifestPath;

    // NOTE: 当加载了GolangPackage的时候，Dependencies和Metadata尚未注册，
    // 因为WorkspaceManager的加载流程是：
    // 1. 扫描哪些插件需要使用
    // 2. 加载这些被标记的插件
    // 3. 扫描依赖
    // 4. 配置包

    // 为什么要把扫描依赖和配置包放在加载插件之后？
    // 为了应对一些极端情况：有些包只会在某些特定环境下启用（常见的就是操作系统相关的库）

    public Dictionary<string, PackageReference> Dependencies { get; init; } = [];
    public Dictionary<string, object>           Metadata     { get; init; } = [];
}