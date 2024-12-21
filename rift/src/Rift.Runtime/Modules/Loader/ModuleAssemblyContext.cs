using System.Runtime.Loader;

namespace Rift.Runtime.Modules.Loader;

internal class ModuleAssemblyContext : AssemblyLoadContext
{
    public ModuleAssemblyContext() : base(true)
    {
        Resolving += (_, args) =>
        {
            var asm = AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName().Name == args.Name);

            return asm;
        };
    }
}