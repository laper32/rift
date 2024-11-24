// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.Serialization;
using System.Text.Json.Serialization;

namespace Rift.Runtime.API.Schema;

public class TomlTask
{
    [JsonPropertyName("about")]
    public string? About { get; set; }

    [JsonPropertyName("heading")]
    public string? Heading { get; set; }

    [JsonPropertyName("is_command")]
    public bool IsCommand { get; set; }

    [JsonPropertyName("before_help")]
    public string? BeforeHelp { get; set; }

    [JsonPropertyName("after_help")]
    public string? AfterHelp  { get; set; }

    /// <summary>
    /// 给一些极端情况用的，一般来说都是用下面的<seealso cref="SubTasks"/>
    /// </summary>
    [JsonPropertyName("parent")]
    public string? Parent { get; set; }

    [JsonPropertyName("sub_tasks")]
    public List<string>? SubTasks { get; set; }

    /// <summary>
    /// 就是Make的Task序列。
    /// </summary>
    [JsonPropertyName("run_tasks")]
    public List<string>? RunTasks { get; set; }

    [JsonPropertyName("args")]
    public List<TomlTaskArg>? Args { get; set; }
}

public class TomlTaskArg
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }

    [JsonPropertyName("short")]
    public char? Short { get; set; }

    [JsonPropertyName("help")]
    public string? Help { get; set; }

    [JsonPropertyName("default")]
    public object? Default { get; set; }

    [JsonPropertyName("conflict_with")]
    public List<string>? ConflictWith { get; set; }

    [JsonPropertyName("help_heading")]
    public string? HelpHeading { get; set; }
}