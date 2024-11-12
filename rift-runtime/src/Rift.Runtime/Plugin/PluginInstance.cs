using System.Reflection;

namespace Rift.Runtime.Plugin;

internal class PluginInstance(PluginIdentity identity, Assembly entry)
{
    public PluginIdentity Identity { get; }
}