using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

public class TomlTask
{
    [DataMember(Name = "about")]
    public string? About { get; set; }

    [DataMember(Name = "is_command")]
    public bool IsCommand { get; set; }

    [DataMember(Name = "parent")]
    public string? Parent { get; set; }

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