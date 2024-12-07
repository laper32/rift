// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks;

public interface IDependentTask
{
    string Name       { get; }
    bool   IsRequired { get; }
}

internal class DependentTask : IDependentTask
{
    public DependentTask(string name, bool required)
    {
        ArgumentNullException.ThrowIfNull(name, nameof(name));
        Name       = name;
        IsRequired = required;
    }

    public string Name       { get; }
    public bool   IsRequired { get; }
}