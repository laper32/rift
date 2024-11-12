namespace Rift.Runtime.Plugin;

internal class PluginInstance(PluginIdentity identity)
{
    public PluginIdentity Identity { get; init; } = identity;

}