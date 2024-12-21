using System.Diagnostics;

namespace Rift.Runtime.Modules.Fundamental;

internal record ModuleSharedAssemblyInfo(string Path, FileVersionInfo Info, DateTime LastWriteDate);

internal class ModuleIdentity
{
    public ModuleIdentity(ModuleType moduleType = ModuleType.Runtime)
    {
        ModuleType = moduleType;
    }

    public ModuleIdentity(string entryPath, ModuleType moduleType = ModuleType.Kernel)
    {
        EntryPath  = entryPath;
        ModuleType = moduleType;
    }

    public Guid Index { get; init; } = Guid.NewGuid();

    public string     EntryPath  { get; set; } = string.Empty;
    public ModuleType ModuleType { get; init; }
}