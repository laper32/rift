// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Text.Json;

namespace Rift.Runtime.Manifest.Real;

/// <summary>
///     将Manifest中[project]解析后的数据。
/// </summary>
/// <param name="Name"> 项目名 </param>
/// <param name="Authors"> 项目作者 </param>
/// <param name="Version"> 项目版本 </param>
/// <param name="Description"> 项目描述 </param>
/// <param name="Plugins"> 处理以来相关的脚本文件，这里为文件路径。 </param>
/// <param name="Dependencies"> 处理以来相关的脚本文件，这里为文件路径。 </param>
/// <param name="Configure"> 处理配置相关的脚本文件，这里为文件路径。 </param>
/// <param name="Target">
///     和下文的 <see cref="Members" />, <see cref="Exclude" /> 互斥：<br />
///     如果出现了该项，则下文的 <see cref="Members" />, <see cref="Exclude" /> 会被忽略 (A.K.A. 永远为空)。<br />
///     用于表达单个项目，例： <br />
///     <code>
///         [project]
///         name = "rift"
/// 
///         [target]
///         name = "rift-bin"
///         type = "bin"
///     </code>
/// </param>
/// <param name="Members">
///     和上文的 <see cref="Target" /> 互斥。 <br />
///     如果出现了该项，则上文的 <see cref="Target" /> 会被忽略 (A.K.A. 永远为空)。<br />
///     用于表达多个项目，例： <br />
///     <code>
///         [project]
///         name = "rift"
///         members = ["a", "b", "c", "d"]
///         exclude = ["e", "f"]
///     </code>
///     <br />
///     和<seealso cref="Exclude" />成对出现。
/// </param>
/// <param name="Exclude">
///     和上文的 <see cref="Target" /> 互斥。 <br />
///     如果出现了该项，则上文的 <see cref="Target" /> 会被忽略 (A.K.A. 永远为空)。<br />
///     用于表达多个项目，例： <br />
///     <code>
///         [project]
///         name = "rift"
///         members = ["a", "b", "c", "d"]
///         exclude = ["e", "f"]
///     </code>
///     <br />
///     和<seealso cref="Members" />成对出现。
/// </param>
internal sealed record ProjectManifest(
    string                          Name,
    List<string>                    Authors,
    string                          Version,
    string                          Description,
    string?                         Plugins,
    string?                         Dependencies,
    string?                         Configure,
    TargetManifest?                 Target,
    List<string>?                   Members,
    List<string>?                   Exclude,
    Dictionary<string, JsonElement> Others);