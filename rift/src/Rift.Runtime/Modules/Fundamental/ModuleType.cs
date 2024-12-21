namespace Rift.Runtime.Modules.Fundamental;

/// <summary>
///     模块类型
/// </summary>
internal enum ModuleType
{
    /// <summary>
    ///     内核模块
    /// </summary>
    Kernel,

    /// <summary>
    ///     运行时模块，可以来自于插件，也可以来自于其他地方
    /// </summary>
    Runtime
}