// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Commands;

/// <summary>
/// See https://docs.rs/clap/latest/clap/enum.ArgAction.html for more details. <br/>
///
/// Just make sure all sites are equivalent. <br/>
/// <remarks>
/// This is not clap! We just use subset of it.
/// </remarks>
/// </summary>
internal enum ArgAction
{
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
    Help,
    HelpShort,
    HelpLong,
    Version,   
}