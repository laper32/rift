#ifndef RIFT_CORE_MODULEDESCRIPTOR_H
#define RIFT_CORE_MODULEDESCRIPTOR_H

#include <string>
#include <vector>

namespace rift {
/*
    public class PluginDescriptor(string instancePath, string pluginPath)
    {
        public string Name { get; set; } = "Unknown";
        public string Author { get; set; } = "Unknown";
        public string Version { get; set; } = "0.0.0.0";
        public string Url { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;

        public string Identifier => Path.GetFileNameWithoutExtension(pluginPath);
        public Guid UniqueId { get; set; }
        public string InstancePath => instancePath;
        public string PluginPath => pluginPath;
        public int ThreadId => Environment.CurrentManagedThreadId;
    }
*/
struct ModuleDescriptor
{
    std::string name;
    std::string author;
    std::string version;
    std::string url;
    std::string description;

};

} // namespace rift

#endif // !RIFT_CORE_MODULEDESCRIPTOR_H
