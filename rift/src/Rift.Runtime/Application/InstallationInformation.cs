namespace Rift.Runtime.Application;

/// <summary>
/// Represents the installation information of an application.
/// </summary>
/// <param name="installationPath">The path where the application is installed.</param>
/// <param name="executablePath">The path to the application's executable file.</param>
public class InstallationInformation(string installationPath, string executablePath)
{
    /// <summary>
    /// Gets the path where the application is installed.
    /// </summary>
    public string InstallationPath { get; init; } = installationPath;

    /// <summary>
    /// Gets the path to the application's executable file.
    /// </summary>
    public string ExecutablePath { get; init; } = executablePath;

    /// <summary>
    /// Gets the path to the application's bin directory.
    /// </summary>
    public string BinPath => Path.Combine(InstallationPath, "bin");

    /// <summary>
    /// Gets the path to the application's lib directory.
    /// </summary>
    public string LibPath => Path.Combine(InstallationPath, "lib");

    /// <summary>
    /// Gets the path to the application's plugins directory.
    /// </summary>
    public string PluginsPath => Path.Combine(InstallationPath, "plugins");
}
