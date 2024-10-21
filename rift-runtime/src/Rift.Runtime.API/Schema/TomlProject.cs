using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

// ReSharper disable once IdentifierTypo
public class TomlProject
{
    [DataMember(Name = "name")] public string Name { get; set; } = null!;

    [DataMember(Name = "authors")] public List<string> Authors { get; set; } = null!;

    [DataMember(Name = "version")] public string Version { get; set; } = null!;
    [DataMember(Name = "description")]
    public string? Description { get; set; }
    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }
    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
    [DataMember(Name = "metadata")]
    public string? Metadata { get; set; }

    [DataMember(Name = "members")]
    public List<string>? Members { get; set; }
    [DataMember(Name = "exclude")]
    public List<string>? Exclude { get; set; }
}
