using System.Runtime.Serialization;

namespace Rift.Runtime.API.Schema;

public class TomlPlugin
{
    [DataMember(Name = "name")]
    public required string Name { get; set; }

    [DataMember(Name = "version")]
    public required string Version { get; set; }
    
    [DataMember(Name = "authors")]
    public required List<string> Authors { get; set; }
    
    [DataMember(Name = "description")]
    public string? Description { get; set; }
    
    [DataMember(Name = "configure")]
    public string? Configure { get; set; }
    
    [DataMember(Name = "dependencies")]
    public string? Dependencies { get; set; }
}
