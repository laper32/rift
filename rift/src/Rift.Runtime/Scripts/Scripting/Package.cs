using Rift.Runtime.Workspace.Fundamental;
using Rift.Runtime.Workspace.Managers;

namespace Rift.Runtime.Scripts.Scripting;

// ReSharper disable UnusedMember.Global
public static class Package
{
    /// <summary>
    ///     为包进行配置<br />
    ///     <remarks>
    ///         只能在脚本中调用该函数！
    ///     </remarks>
    /// </summary>
    /// <param name="configure"> </param>
    public static void Configure(Action<PackageConfiguration> configure)
    {
        WorkspaceManager.ConfigurePackage(configure);
    }
}