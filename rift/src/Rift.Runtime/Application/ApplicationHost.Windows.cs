using System.Runtime.InteropServices;
using System.Text;

namespace Rift.Runtime.Application;

public sealed partial class ApplicationHost
{
    private const int MaxPath = 260;

    // https://learn.microsoft.com/en-us/windows/desktop/api/shlwapi/nf-shlwapi-pathfindonpathw
    // https://www.pinvoke.net/default.aspx/shlwapi.PathFindOnPath
    // ReSharper disable once StringLiteralTypo
    [DllImport("shlwapi.dll", CharSet = CharSet.Unicode, SetLastError = false)]
    private static extern bool PathFindOnPath([In] [Out] StringBuilder pszFile, [In] string[]? ppszOtherDirs);

    /// <summary>
    ///     Gets the full path of the given executable filename as if the user had entered this
    ///     executable in a shell. So, for example, the Windows PATH environment variable will
    ///     be examined. If the filename can't be found by Windows, null is returned. <br />
    ///     see: https://stackoverflow.com/questions/3855956/check-if-an-executable-exists-in-the-windows-path
    /// </summary>
    /// <param name="exeName"> The name of the executable. </param>
    /// <returns> The full path if successful, or null otherwise. </returns>
    /// <exception cref="ArgumentException"> Thrown when the executable name is too long. </exception>
    private static string? GetPathFromPathVariableWindows(string exeName)
    {
        if (exeName.Length >= MaxPath)
        {
            throw new ArgumentException(
                $"The executable name '{nameof(exeName)}' must have less than {MaxPath} characters."
            );
        }

        var builder = new StringBuilder(exeName, MaxPath);
        return PathFindOnPath(builder, null) ? builder.ToString() : null;
    }
}