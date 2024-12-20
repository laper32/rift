// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Kokuban;

namespace Rift.Runtime.Fundamental;

/// <summary>
///     Provides methods for writing formatted messages to the console.
/// </summary>
public class Tty
{
    /// <summary>
    ///     Writes a warning message to the console.
    /// </summary>
    /// <param name="message"> The warning message to write. </param>
    public static void Warning(string message = "")
    {
        Console.WriteLine($"{Chalk.Bold.Yellow["warning"]}: {message}");
    }

    /// <summary>
    ///     Writes an error message to the console.
    /// </summary>
    /// <param name="message"> The error message to write. </param>
    public static void Error(string message = "")
    {
        Console.WriteLine($"{Chalk.Bold.Red["error"]}: {message}");
    }

    /// <summary>
    ///     Writes an error message and exception details to the console.
    /// </summary>
    /// <param name="e"> The exception to write. </param>
    /// <param name="message"> The error message to write. </param>
    public static void Error(Exception e, string message = "")
    {
        Console.WriteLine(string.IsNullOrEmpty(message)
            ? $"{Chalk.Bold.Red["error"]}: {e}"
            : $"{Chalk.Bold.Red["error"]}: {message}{Environment.NewLine}{e}"
        );
    }

    /// <summary>
    ///     Writes a message to the console.
    /// </summary>
    /// <param name="message"> The message to write. </param>
    public static void WriteLine(string message = "")
    {
        Console.WriteLine(message);
    }
}