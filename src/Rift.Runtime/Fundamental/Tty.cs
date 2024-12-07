// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Kokuban;

namespace Rift.Runtime.Fundamental;

public class Tty
{
    public static void Warning(string message = "")
    {
        Console.WriteLine($"{Chalk.Bold.Yellow["warn"]}: {message}");
    }

    public static void Error(string message = "")
    {
        Console.WriteLine($"{Chalk.Bold.Red["error"]}: {message}");
    }

    public static void Error(Exception e, string message = "")
    {
        Console.WriteLine($"{Chalk.Bold.Red["error"]}: {message}{Environment.NewLine}{e}");
    }

    public static void WriteLine(string message = "")
    {
        Console.WriteLine(message);
    }
}