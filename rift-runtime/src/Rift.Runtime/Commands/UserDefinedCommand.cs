// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Commands;


internal record UserDefinedCommand(
    string                      Name,
    string?                     About,
    string?                     BeforeHelp,
    string?                     AfterHelp,
    string?                     Parent,
    List<string>                Subcommands,
    string                      PackageName,
    List<UserDefinedCommandArg> Args);

internal record UserDefinedCommandArg(
    string Name,
    char? Short,
    string? Description,
    object? Default,
    List<string> ConflictWith,
    string? Heading);
