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
    /// <summary>
    /// 标记该包和某个特定的包使用相同的引用. <br/>
    ///
    /// 该函数会在<see cref="PackageReference.Attributes"/>处创建`Ref`字段, 其为string. <br/>
    /// <param name="packageName">对应的包名, 这里不检查是否存在.</param>
    /// </summary>
    public static PackageReference Ref(this PackageReference self, string packageName)
    {
        // 如果已经ref workspace了, 直接跳过
        if (self.IsRefWorkspace())
        {
            return self;
        }

        // Ref这个标签一定是string
        self.Attributes.Add("Ref", packageName);

        return self;
    }

    /// <summary>
    /// 判断该引用是否和某个特定的包相同
    /// </summary>
    /// <param name="self"> </param>
    /// <param name="packageName">对应的包名, 这里不检查是否存在.</param>
    /// <returns>True if same, false otherwise.</returns>
    public static bool IsRef(this PackageReference self, string packageName)
    {
        if (!self.Attributes.TryGetValue("Ref", out var refPackage))
        {
            return false;
        }

        return refPackage is string ret && ret.Equals(packageName, StringComparison.OrdinalIgnoreCase);
    }

    /// <summary>
    /// 判断该包是否引用Workspace. <br/>
    /// </summary>
    /// <returns>True if references workspace, false otherwise.</returns>
    public static bool IsRefWorkspace(this PackageReference self)
    {
        if (!self.Attributes.TryGetValue("RefWorkspace", out var refWorkspace))
        {
            return false;
        }

        return refWorkspace is true;
    }

    /// <summary>
    /// 标记该包引用Workspace. <br/>
    ///
    /// 该函数会在<see cref="PackageReference.Attributes"/>处创建`RefWorkspace`字段, 其为bool. <br/>
    /// </summary>
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
