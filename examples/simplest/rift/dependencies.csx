// this will be occur when Rift.Cxx is ready.
class CxxDependency
{
    public string Name { get; init; }
    public string Version { get; init; }
    public Dictionary<string, object> Attributes;

    public CxxDependency(string name)
    {
        Name = name;
        Version = "";
        Attributes = new Dictionary<string, object>();
    }

    public CxxDependency(string name, string version)
    {
        Name = name;
        Version = version;
        Attributes = new Dictionary<string, object>();
    }
}

Dependencies.Add([
    new CxxDependency("boost", "1.82.0"),
    new CxxDependency("fmt", "7.1.3"),
    new CxxDependency("nlohmann_json", "3.9.1"),
    new CxxDependency("spdlog", "1.9.2"),
    new CxxDependency("zlib", "1.2.11")
    new CxxDependency("hl2sdk") {
        Attributes = {
            {"path", "path/to/hl2sdk"}
        }
    }
]);