namespace Rift.Runtime.Application;

public class InstallationInformation(string installationPath, string executablePath)
{
    public string InstallationPath { get; init; } = installationPath;

    public string ExecutablePath { get; init; } = executablePath;
    public string BinPath        => Path.Combine(InstallationPath, "bin");
    public string LibPath        => Path.Combine(InstallationPath, "lib");
    public string PluginsPath    => Path.Combine(InstallationPath, "plugins");
}