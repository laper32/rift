namespace Rift.Runtime.Application;

/// <summary>
/// Represents user information and provides paths for user-specific directories.
/// </summary>
/// <param name="userPath">The base path for the user.</param>
public class UserInformation(string userPath)
{
    /// <summary>
    /// Gets the base path for the user.
    /// </summary>
    public string UserPath { get; init; } = userPath;

    /// <summary>
    /// Gets the path to the user's bin directory.
    /// </summary>
    public string BinPath => Path.Combine(UserPath, "bin");

    /// <summary>
    /// Gets the path to the user's lib directory.
    /// </summary>
    public string LibPath => Path.Combine(UserPath, "lib");

    /// <summary>
    /// Gets the path to the user's plugins directory.
    /// </summary>
    public string PluginsPath => Path.Combine(UserPath, "plugins");
}
