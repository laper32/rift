using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

public class TomlTask
{
    [DataMember(Name = "about")]
    public string? About { get; set; }

    [DataMember(Name = "heading")]
    public string? Heading { get; set; }

    [DataMember(Name = "is_command")]
    public bool IsCommand { get; set; }

    [DataMember(Name = "before_help")]
    public string? BeforeHelp { get; set; }

    [DataMember(Name = "after_help")]
    public string? AfterHelp  { get; set; }

    /// <summary>
    /// 给一些极端情况用的，一般来说都是用下面的<seealso cref="SubTasks"/>
    /// </summary>
    [DataMember(Name = "parent")]
    public string? Parent { get; set; }

    /// <summary>
    /// 如果标注了这个task是指令，那么底下的所有subtask都是指令，会直接覆盖掉原本的<seealso cref="IsCommand"/>
    /// </summary>
    [DataMember(Name = "sub_tasks")]
    public List<string>? SubTasks { get; set; }

    /// <summary>
    /// 就是Make的Task序列。
    /// </summary>
    [DataMember(Name = "run_tasks")]
    public List<string>? RunTasks { get; set; }

    [DataMember(Name = "args")]
    public List<TomlTaskArg>? Args { get; set; }
}

public class TomlTaskArg
{
    [DataMember(Name = "name")]
    public required string Name { get; set; }

    [DataMember(Name = "short")]
    public char? Short { get; set; }

    [DataMember(Name = "description")]
    public string? Description { get; set; }

    [DataMember(Name = "default")]
    public object? Default { get; set; }

    [DataMember(Name = "conflict_with")]
    public List<string>? ConflictWith { get; set; }

    [DataMember(Name = "heading")]
    public string? Heading { get; set; }
}