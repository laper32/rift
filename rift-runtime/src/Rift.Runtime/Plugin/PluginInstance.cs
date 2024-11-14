using System.Reflection;
using Rift.Runtime.API.Plugin;

namespace Rift.Runtime.Plugin;

internal class PluginInstance(PluginContext context)
{
    private readonly PluginIdentity _identity = context.Identity;
    private readonly Assembly?      _entry    = context.Entry;
    public           RiftPlugin?    Instance { get; private set; }

    // TODO: 二进制加载逻辑挪到ModuleSystem.
    public bool Init()
    {
        return true;
    }
}