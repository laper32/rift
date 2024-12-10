using System.Diagnostics;
using Rift.Runtime.Fundamental;

namespace Rift.Go.Fundamental;

public class GolangEnvironment
{
    private static GolangEnvironment _instance = null!;

    // Unix: /bin/go, Windows: $PATH/go.exe
    private readonly string _goExe;
    private readonly string _goVersion;

    public GolangEnvironment()
    {
        var goExecutableFileName = "go";
        if (OperatingSystem.IsWindows())
        {
            goExecutableFileName += ".exe";
        }

        _goExe = ApplicationHost.GetPathFromPathVariable(goExecutableFileName) ?? "";

        if (string.IsNullOrEmpty(_goExe))
        {
            _goVersion = ""; //这时候版本就完全不知道了，只能靠Go.Version环境变量指定用什么版本
        }
        else
        {
            var process = new Process
            {
                StartInfo = new ProcessStartInfo
                {
                    FileName               = _goExe,
                    Arguments              = "env GOVERSION",
                    RedirectStandardOutput = true,
                    RedirectStandardError  = true,
                    UseShellExecute        = false,
                    CreateNoWindow         = true
                }
            };
            process.Start();

            _goVersion = process.StandardError.ReadToEnd().Length > 0
                ? ""
                :
                // go的版本号是go+对应的版本号，比如说go1.22.2.
                // 我们拿到版本号后就直接扔掉前面的go就行。
                process.StandardOutput.ReadToEnd().TrimStart(['g', 'o']);
            process.WaitForExit();
        }

        _instance = this;
    }

    public static string ExecutablePath => _instance._goExe;
    public static string Version        => _instance._goVersion;
}