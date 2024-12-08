// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

// ReSharper disable UnusedMember.Global
public class Tasks
{
    /// <summary>
    ///     为脚本系统注册Task。 <br />
    ///     <remarks>
    ///         该函数只能是脚本调用！
    ///     </remarks>
    /// </summary>
    /// <param name="name"> </param>
    /// <param name="predicate"> </param>
    public static void Register(string name, Action<ITaskConfiguration> predicate)
    {
        TaskManager.RegisterTask(name, predicate);
    }
}