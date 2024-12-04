using System.Text.Json.Serialization;
using Rift.Runtime.Scripting;

namespace Rift.Go.Scripting;

// TODO: 准备移除，统一到PackageReference
[Serializable]
public class GolangImport : IPackageImportDeclarator
{
    [JsonIgnore]
    private bool _refWorkspace;

    public GolangImport(string name)
    {
        Name       = name;
        Version    = "";
        Attributes = [];
    }

    public GolangImport(string name, string version)
    {
        Name       = name;
        Version    = version;
        Attributes = [];
    }

    public string                     Version    { get; }
    public Dictionary<string, object> Attributes { get; init; }

    public string Name { get; init; }

    /// <summary>
    ///     指定该包使用根目录下的声明. <br />
    ///     该标记拥有最高优先级. <br />
    ///     如果你的项目比较复杂(如Workspace-Project-Target结构), 则请使用 <see cref="Ref" />声明具体需要使用的项目.
    /// </summary>
    /// <returns> Instance this </returns>
    public GolangImport RefWorkspace()
    {
        if (_refWorkspace)
        {
            return this;
        }

        Attributes.Add("RefWorkspace", true);
        _refWorkspace = true;

        return this;
    }

    public GolangImport Ref(string packageName)
    {
        Attributes.Add("Ref", packageName);
        return this;
    }
}