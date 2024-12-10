namespace Rift.Runtime.Fundamental;

public sealed partial class ApplicationHost
{
    private const char PathSeparator = ':';

    private static IEnumerable<string> ParsePathsFromPathVariable()
    {
        var path  = Environment.GetEnvironmentVariable("PATH")!;
        var paths = path.Split(PathSeparator);
        return paths.Where(x => !string.IsNullOrWhiteSpace(x)).Distinct().ToList();
    }

    private static string? GetPathFromPathVariableUnix(string exeName)
    {
        var paths = ParsePathsFromPathVariable();

        // 这里只看出现的优先级，如果出现了多个结果我们就只认第一个。
        // 而且，我们不会判断该文件是否为可执行文件，需要用户自行处理。
        var possibleExecutable = paths.Select(path => Path.Combine(path, exeName)).Where(File.Exists).ToList();

        // 字面意思了：如果没有的话就直接返回空（aka：返回字符串为空）
        // 否则我们只看第一个出现的元素
        // 不考虑除了第一个元素之外的所有元素：因为大多数情况下PATH你也会只关心第一个，或者说，你都进PATH了，
        // 那么你是有义务确保出现在PATH的可执行文件一定是唯一的。
        return possibleExecutable.Count > 0 ? possibleExecutable.First() : null;
    }
}