namespace Rift.Go.Workspace;

public class GolangConfiguration
{
    internal Dictionary<string, object> Attributes { get; init; } = [];
}

public static class GolangConfigurationExtensions
{
    public static string GetProxy(this GolangConfiguration self)
    {
        if (!self.Attributes.TryGetValue("Proxy", out var proxy))
        {
            return "";
        }

        return proxy as string ?? "";
    }

    public static GolangConfiguration SetProxy(this GolangConfiguration self, string value)
    {
        self.Attributes["Proxy"] = value;
        return self;
    }

    public static string GetVersion(this GolangConfiguration self)
    {
        if (!self.Attributes.TryGetValue("Version", out var version))
        {
            return "";
        }

        return version as string ?? "";
    }

    public static GolangConfiguration SetVersion(this GolangConfiguration self, string value)
    {
        self.Attributes["Version"] = value;
        return self;
    }
}