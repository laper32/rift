using System.Runtime.Serialization;
using System.Text;

namespace Rift.Runtime.API.Schema;

// ReSharper disable IdentifierTypo
// ReSharper disable StringLiteralTypo
public class TomlFolder
{
    [DataMember(Name = "name")]
    public string? Name { get; set; }

    [DataMember(Name = "members")]
    public required List<string> Members { get; set; }

    [DataMember(Name = "exclude")]
    public List<string> Exclude { get; set; } = [];

}