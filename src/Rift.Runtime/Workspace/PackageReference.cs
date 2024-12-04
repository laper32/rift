namespace Rift.Runtime.Workspace;

public class PackageReference
{
    public string                     Name       { get; init; }
    public string                     Version    { get; set; }
    public Dictionary<string, object> Attributes { get; init; } = [];
    public PackageReference(string name)
    {
        Name = name;
        Version = "";
        Attributes = [];
    }

    public PackageReference(string name, string version)
    {
        Name       = name;
        Version    = version;
        Attributes = [];
    }
}

public static class PackageReferenceExtensions
{
    public static PackageReference Ref(this PackageReference self, string packageName)
    {
        self.Attributes.Add("Ref", packageName);

        return self;
    }

    public static PackageReference RefWorkspace(this PackageReference self)
    {
        if (self.Attributes.TryGetValue("RefWorkspace", out var refWorkspace))
        {
            if (refWorkspace is not bool)
            {
                self.Attributes["RefWorkspace"] = true;
            }

            return self;
        }

        self.Attributes["RefWorkspace"] = true;
        return self;
    }
}
