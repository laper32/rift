Package.Configure(config =>
{
    config.GolangVersion("1.22.2");
});

// rift hello
Tasks.Register("rift.hello", (configure) =>
{
    configure
        .SetDeferException(true)
        .SetIsCommand(true);
});

// rift hello nest
Tasks.Register("rift.hello.nest", configure =>
{
    configure
        .SetIsCommand(true)
        .AddAction(() =>
            {
                Console.WriteLine("Hello, Nest!");
            }
        );
});