// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Tasks.Configuration;
using Rift.Runtime.Tasks.Managers;

namespace Rift.Runtime.Tasks.Scripting;

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
    public static void Register(string name, Action<TaskConfiguration> predicate)
    {
        TaskManager.RegisterTask(name, predicate);
    }
}