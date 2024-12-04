namespace Rift.Runtime.Commands;

internal class UserCommandEntry(string name)
{
    public string                 Name     { get; set; }  = name;
    public List<UserCommandEntry> Children { get; init; } = [];

    public string TaskName { get; set; } = null!;

    public UserCommandEntry AddChild(string childName)
    {
        var childNode = new UserCommandEntry(childName);
        Children.Add(childNode);
        return childNode;
    }
}