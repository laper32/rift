using Rift.Go.Workspace;
using System.Net;
using System.Net.Http.Json;

namespace Rift.Go.Generate;

internal class GolangGenerateService
{
    private readonly SocketsHttpHandler _httpHandler;

    public GolangGenerateService()
    {
        _httpHandler = new SocketsHttpHandler
        {
            AllowAutoRedirect              = true,
            AutomaticDecompression         = DecompressionMethods.All,
            EnableMultipleHttp2Connections = true,
            ConnectTimeout                 = TimeSpan.FromSeconds(10),
        };
        Instance = this;
    }

    internal static GolangGenerateService Instance { get; private set; } = null!;

    public static void PerformGolangGenerate()
    {
        var          goProxy = Environment.GetEnvironmentVariable("GOPROXY") ?? "";
        List<string> split   = [];
        if (!string.IsNullOrEmpty(goProxy))
        {
            split.AddRange(goProxy.Split(','));
        }

        var proxyUrl = new Uri("https://proxy.golang.org"); // eg: https://proxy.golang.org or https://goproxy.cn

        if (split.Count > 0)
        {
            var proxySiteRaw = split[0];
            if (!string.IsNullOrEmpty(proxySiteRaw))
            {
                proxyUrl = new Uri(proxySiteRaw);
            }
        }

        var httpClient = new HttpClient(Instance._httpHandler, false)
        {
            BaseAddress = proxyUrl,
            Timeout = TimeSpan.FromSeconds(10),
            DefaultRequestVersion = HttpVersion.Version20,
            DefaultVersionPolicy = HttpVersionPolicy.RequestVersionOrLower,
        };
        httpClient.DefaultRequestHeaders.Add("User-Agent", "Rift.Go.Generate");

        var queryResult = Task.Run(async () =>
        {
            // https://proxy.golang.org/github.com/laper32/goose/@v0.1.0
            // https://proxy.golang.org/github.com/laper32/goose/@latest
            var url = $"github.com/laper32/goose/@latest";
            var response = await httpClient.GetAsync(url);
            response.EnsureSuccessStatusCode();
            var result = await response.Content.ReadAsStringAsync() ??
                         throw new InvalidDataException("Invalid upstream data");
            return result;
        }).Result;
        Console.WriteLine($"{queryResult}");
    }

    internal string GenerateGoModString()
    {
        return """

               module <Workspace.Name>

               go <Go.Version>

               require (
               	<DirectRequires>
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
}