using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;


// ReSharper disable once IdentifierTypo
public class TomlWorkspace
{
    [DataMember(Name = "members")]
    public List<string>? Members { get; set; }
    [DataMember(Name = "exclude")]
    public List<string>? Exclude { get; set; }
    [DataMember(Name = "plugins")]
    public string? Plugins { get; set; }
    [DataMember(Name = "metadata")]
    public string? Metadata { get; set; }
    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
}
