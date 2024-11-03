namespace Rift.Runtime.API.Manifest;

public interface IRiftManifest
{
    string       Name        { get; }
    List<string> Authors     { get; }
    string       Version     { get; }
    string?      Description { get; }
    string?      Metadata    { get; }
    string?      Dependencies  { get; }
}