// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Commands;

internal class UserCommand(string name, string packageName)
{
    public string                Name        => name;
    public string?               About       { get; set; }
    public string?               BeforeHelp  { get; set; }
    public string?               AfterHelp   { get; set; }
    public List<UserCommand>?    Subcommands { get; set; }
    public string                PackageName => packageName;
    public List<UserCommandArg>? Args        { get; set; }
}

internal class UserCommandArg(string name)
{
    public string       Name         => name;
    public char?        Short        { get; set; }
    public string?      Description  { get; set; }
    public object?      Default      { get; set; }
    public List<string> ConflictWith { get; init; } = [];
    public string?      Heading      {get;  set; }
    public ArgAction?    Action       { get; set; }
}
