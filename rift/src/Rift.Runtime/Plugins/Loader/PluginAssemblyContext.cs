using System.Runtime.Loader;

namespace Rift.Runtime.Plugins.Loader;

internal class PluginAssemblyContext : AssemblyLoadContext
{
    public PluginAssemblyContext() : base(true)
    {
        Resolving += (_, args) =>
            AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName().Name == args.Name);
    }
}