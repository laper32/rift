// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Fundamental.Script;

internal class ScriptContext(string path)
{
    public string Path { get; init; } = path;

    // 举个例子：C:/Users/user/MyScript.csx，那么所在的ScriptDir就是C:/Users/user
    // 这里的核心目的是得知该脚本文件所在的文件夹，方便操作。
    public string Location { get; init; } = Directory.GetParent(path)!.FullName;

    public string Text { get; init; } = File.ReadAllText(path);
}