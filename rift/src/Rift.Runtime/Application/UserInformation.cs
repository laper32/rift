namespace Rift.Runtime.Application;

public class UserInformation(string userPath)
{
    public string UserPath { get; init; } = userPath;

    public string BinPath     => Path.Combine(UserPath, "bin");
    public string LibPath     => Path.Combine(UserPath, "lib");
    public string PluginsPath => Path.Combine(UserPath, "plugins");
}