using Rift.Generate.Abstractions;
using Rift.Go.Application;
using System.Data;
using System.Net;
using System.Net.Http.Json;
using Rift.Go.Scripting;
using Rift.Go.Workspace.Managers;
using Rift.Go.Workspace.Fundamental;

namespace Rift.Go.Generate;

internal class GolangGenerateService(
    GolangWorkspaceManager workspaceService,
    GolangEnvironment goEnv
    ) : IGenerateListener
{
    private readonly SocketsHttpHandler _httpHandler = new()
    {
        AllowAutoRedirect = true,
        AutomaticDecompression = DecompressionMethods.All,
        EnableMultipleHttp2Connections = true,
        ConnectTimeout = TimeSpan.FromSeconds(10)
    };

    public void OnGenerate()
    {
        PerformGolangGenerate();
    }

    private void PerformGolangGenerate()
    {
        var goProxy = Environment.GetEnvironmentVariable("GOPROXY") ?? "";
        List<string> split = [];
        if (!string.IsNullOrEmpty(goProxy))
        {
            split.AddRange(goProxy.Split(','));
        }

        // see https://go.dev/ref/mod#goproxy-protocol
        var proxyUrl = new Uri("https://proxy.golang.org"); // eg: https://proxy.golang.org or https://goproxy.cn

        if (split.Count > 0)
        {
            var proxySiteRaw = split[0];
            if (!string.IsNullOrEmpty(proxySiteRaw))
            {
                proxyUrl = new Uri(proxySiteRaw);
            }
        }

        var httpClient = new HttpClient(_httpHandler, false)
        {
            BaseAddress = proxyUrl,
            Timeout = TimeSpan.FromSeconds(10),
            DefaultRequestVersion = HttpVersion.Version20,
            DefaultVersionPolicy = HttpVersionPolicy.RequestVersionOrLower
        };
        httpClient.DefaultRequestHeaders.Add("User-Agent", "Rift.Go.Generate");

        workspaceService.Packages.ForEach(pkg =>
        {
            foreach (var reference in pkg.Dependencies.Values)
            {
                // 有没有标latest的API是完全不一样的，但目前我们只需要看latest所在的版本号就行。
                // 至于非latest版本号后续怎么处理，目前不管，后续再搞。
                // TODO: 这玩意肯定要搞缓存的
                string actualVersion;
                if (reference.Version.Equals("latest", StringComparison.OrdinalIgnoreCase))
                {
                    actualVersion = Task.Run(async () =>
                    {
                        var query = $"{reference.Name}/@latest";
                        var response = await httpClient.GetAsync(query);
                        response.EnsureSuccessStatusCode();
                        var result = await response.Content.ReadFromJsonAsync<PackageVersionInfo>() ??
                                     throw new InvalidDataException("Invalid upstream data");
                        return result.Version.TrimStart('v');
                    }).Result;
                }
                else
                {
                    actualVersion = reference.Version;
                }

                reference.Version = actualVersion;
            }

            var targetGoModPath = pkg.Configuration.GetWriteModToPath();
            if (string.IsNullOrEmpty(targetGoModPath))
            {
                targetGoModPath = pkg.Root;
            }

            targetGoModPath = Path.Join(targetGoModPath, "go.mod");
            File.WriteAllText(targetGoModPath, GenerateGoModString(pkg));
        });
    }

    internal string GenerateGoModString(GolangPackage package)
    {
        var goExeVersion = goEnv.Version;
        var packageGoVersion = package.Configuration.GetGolangVersion();
        var globalEnvGoVersion = Environment.GetEnvironmentVariable("Go.Version") ?? "";
        if (string.IsNullOrEmpty(goExeVersion) && string.IsNullOrEmpty(packageGoVersion) &&
            string.IsNullOrEmpty(globalEnvGoVersion))
        {
            throw new DataException("Must specify go version first!");
        }

        string goVersion;
        // 如果包里面规定了go的版本，就直接用，其为最高优先级
        if (!string.IsNullOrEmpty(packageGoVersion))
        {
            goVersion = packageGoVersion;
        }
        // 如果没有规定，看全局变量
        else if (!string.IsNullOrEmpty(globalEnvGoVersion))
        {
            goVersion = globalEnvGoVersion;
        }
        // 都没有，就看系统默认的go版本
        else
        {
            goVersion = goExeVersion;
        }

        var goReferences = package.Dependencies.Values.Select(dependency => $"{dependency.Name} v{dependency.Version}")
            .ToList();

        var goReferencesStr = string.Join($"{Environment.NewLine}", goReferences);

        return $"""
                   module {package.Name}

                   go {goVersion}

                   require (
                   	{goReferencesStr}
                   )
                   """;
    }

    internal string GenerateGoWorkString()
    {
        return """
                  go <Go.Version>

                  use (
                  	<LocalPackages>
                  )

                  """;
    }

    private record PackageVersionInfo(string Version, string Time);

}