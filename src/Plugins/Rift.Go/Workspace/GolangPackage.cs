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
    // 为了应对一些极端情况：有些包只会在某些特定环境下启用（常见的就是操作系统相关的库，或者某些库只在某个特定环境下启用）
    // 比如说一个库同时支持.NET8, .NET9，然后.NET8环境下库版本肯定和.NET9的不一样
    // 这时候runtime builtin是不可能满足这些需求的。
    // (环境变量行不行？不行，因为不管怎么样还是会回到这个问题上：环境变量）
    // 现在还是草稿，会持续更新，直到定稿为止。

    public Dictionary<string, PackageReference> Dependencies  => package.Dependencies;
    public PackageConfiguration                 Configuration => package.Configuration;
}