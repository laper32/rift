namespace Rift.Runtime.Tasks;

internal static class RiftTaskExtensions
{
    public static RiftTask AddDependency(this RiftTask self, string name, bool required = true)
    {
        if (self.Dependencies.Any(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase)))
        {
            throw new InvalidOperationException($"The task `{self.Name}` already have a dependency on `{name}`");
        }

        self.Dependencies.Add(new DependentTask(name, required));
        
        return self;
    }
}